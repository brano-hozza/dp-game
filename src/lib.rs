use std::{str::FromStr, sync::Arc};

use alloy::{
    network::{EthereumWallet, TransactionBuilder},
    primitives::{address, Address, Uint},
    providers::{self, Provider, ProviderBuilder, RootProvider},
    rpc::types::Filter,
    signers::local::PrivateKeySigner,
    sol_types::sol,
    transports::http::{Client, Http},
};
use alloy_sol_types::SolEvent;
use bevy::{prelude::*, tasks::futures_lite::StreamExt};
use bevy_tokio_tasks::TokioTasksRuntime;
use common::systems::print_position_system;
use map::{resources::GameMap, systems::update_map, MapTile};
use player::systems::move_player_system;
use score::{
    components::GainScoreEvent,
    systems::{check_chests_system, display_score_gain_system},
};
use GoldContract::GoldContractInstance;
use SwordContract::SwordContractInstance;
// alloy - alternative to web3

pub mod common;
pub mod map;
pub mod player;
pub mod score;
pub mod ui;

const MAX_TESTNET_GAS_PRICE: u128 = 20_000_000_000;

const ALICE_ADDRESS: &str = "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266";
const ALICE_PK: &str = "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";

const BOB_ADDRESS: &str = "0x70997970C51812dc3A010C7d01b50e0d17dc79C8";

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    GoldContract,
    "assets/contracts/Gold.json"
);

type HttpClient = Http<Client>;
type ContractProvider = Arc<RootProvider<Http<Client>>>;

#[derive(Resource)]
pub struct EthProviderResource(Arc<ContractProvider>);

#[derive(Resource)]
pub struct GoldContractResource(Arc<GoldContractInstance<HttpClient, ContractProvider>>);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    SwordContract,
    "assets/contracts/Sword.json"
);

#[derive(Resource)]
pub struct SwordContractResource(Arc<SwordContractInstance<HttpClient, ContractProvider>>);

fn display_balances(runtime: Res<TokioTasksRuntime>, gold_contract_res: Res<GoldContractResource>) {
    let gold_contract = gold_contract_res.0.clone();
    runtime.spawn_background_task(|_ctx| {
        // Move everything into the async block, ensuring all variables are owned.
        async move {
            let accounts: Vec<Address> = vec![
                Address::from_str(ALICE_ADDRESS).unwrap(),
                Address::from_str(BOB_ADDRESS).unwrap(),
            ];

            for account in accounts {
                // Use the contract to make async calls.
                let res = gold_contract.balanceOf(account).call().await.unwrap();
                let balance = res._0;
                println!("Balance of {}: {}", account, balance);
            }
        }
    });
}

fn try_mint_sword(
    runtime: Res<TokioTasksRuntime>,
    provider: Res<EthProviderResource>,
    sword_contract_res: Res<SwordContractResource>,
) {
    let provider = provider.0.clone();
    let sword_contract = sword_contract_res.0.clone();
    runtime.spawn_background_task(|_ctx| {
        // Move everything into the async block, ensuring all variables are owned.
        async move {
            let account_alice = Address::from_str(ALICE_ADDRESS).unwrap();
            let signer: PrivateKeySigner = ALICE_PK.parse().expect("Failed to parse pk");
            let wallet = EthereumWallet::from(signer);

            let chain_id = provider
                .get_chain_id()
                .await
                .expect("Failed to get chain id");

            // Get accout nonce
            let nonce = provider
                .get_transaction_count(account_alice)
                .await
                .expect("Failed to get nonce");

            let tx = sword_contract
                .mint(account_alice, Uint::from(2))
                .into_transaction_request();

            // Use the contract to make async calls.
            let envelope = tx
                .with_nonce(nonce)
                .with_from(account_alice)
                .with_max_fee_per_gas(MAX_TESTNET_GAS_PRICE)
                .with_gas_limit(40_000)
                .with_chain_id(chain_id)
                .with_max_priority_fee_per_gas(1_000_000_000)
                .build(&wallet)
                .await
                .expect("Failed to build tx");

            let pending_tx = provider
                .send_tx_envelope(envelope)
                .await
                .expect("Failed to send tx");

            let receipt = pending_tx
                .get_receipt()
                .await
                .expect("Failed to get receipt");
            println!(
                "Minted sword with tx hash: {} and block number: {}",
                receipt.transaction_hash,
                receipt.block_number.expect("Failed to get block number")
            );

            // Transfer sword from Alice to Bob

            let account_bob = Address::from_str(BOB_ADDRESS).unwrap();

            let tx = sword_contract
                .transferFrom(account_alice, account_bob, Uint::from(2))
                .into_transaction_request();

            let nonce = provider
                .get_transaction_count(account_alice)
                .await
                .expect("Failed to get nonce");

            let envelope = tx
                .with_nonce(nonce)
                .with_from(account_alice)
                .with_max_fee_per_gas(MAX_TESTNET_GAS_PRICE)
                .with_gas_limit(40_000)
                .with_chain_id(chain_id)
                .with_max_priority_fee_per_gas(1_000_000_000)
                .build(&wallet)
                .await
                .expect("Failed to build tx");

            let receipt = provider
                .send_tx_envelope(envelope)
                .await
                .expect("Failed to send tx")
                .get_receipt()
                .await
                .expect("Failed to get receipt");

            println!(
                "Transferred sword with tx hash: {}",
                receipt.transaction_hash
            );
        }
    });
}

pub fn listen_to_transfers(
    runtime: Res<TokioTasksRuntime>,
    provider: Res<EthProviderResource>,
    sword_contract_res: Res<SwordContractResource>,
) {
    let sword_contract = sword_contract_res.0.clone();
    let provider = provider.0.clone();
    runtime.spawn_background_task(|_ctx| {
        // Move everything into the async block, ensuring all variables are owned.
        async move {
            let transfer_filter = Filter::new()
                .address(*sword_contract.address())
                .events([SwordContract::Transfer::SIGNATURE]);

            let Ok(logs) = provider.get_logs(&transfer_filter).await else {
                todo!()
            };

            if let Some(last_event) = logs.last() {
                if let Ok(transfer) = last_event.log_decode::<SwordContract::Transfer>() {
                    println!("Transfer event: {:?}", transfer.address());
                }
            }
        }
    });
}

pub struct DpPlugin;

impl Plugin for DpPlugin {
    fn build(&self, app: &mut App) {
        let _ = env_logger::try_init();

        let json_map: Vec<Vec<u32>> =
            serde_json::from_str(include_str!("../assets/maps/01.json")).unwrap();
        let game_map: Vec<Vec<MapTile>> = json_map
            .iter()
            .map(|row| row.iter().map(|tile| MapTile::from(*tile)).collect())
            .collect();

        let rpc_url = "http://127.0.0.1:8545";

        // Create the provider and wrap it in Arc to share ownership across async blocks.
        let provider = Arc::new(
            ProviderBuilder::new().on_http(rpc_url.parse().expect("Failed to parse rpc url")),
        );

        // Create the gold_contract and wrap it in Arc to allow thread-safe access.
        let gold_contract = GoldContract::new(
            address!("5FbDB2315678afecb367f032d93F642f64180aa3"),
            provider.clone(), // Clone the Arc to share ownership of the provider.
        );

        // Create the sword_contract and wrap it in Arc to allow thread-safe access.
        let sword_contract = SwordContract::new(
            address!("Cf7Ed3AccA5a467e9e704C703E8D87F634fB0Fc9"),
            provider.clone(), // Clone the Arc to share ownership of the provider.
        );

        app.add_plugins(bevy_tokio_tasks::TokioTasksPlugin::default())
            .insert_resource(GameMap(game_map))
            .insert_resource(EthProviderResource(Arc::new(provider)))
            .insert_resource(GoldContractResource(Arc::new(gold_contract)))
            .insert_resource(SwordContractResource(Arc::new(sword_contract)))
            .add_event::<GainScoreEvent>()
            .add_systems(
                Startup,
                (
                    display_balances,
                    listen_to_transfers,
                    try_mint_sword,
                    setup,
                    ui::systems::setup,
                    map::systems::setup,
                    player::systems::setup,
                )
                    .chain(),
            )
            .add_systems(
                Update,
                (
                    print_position_system,
                    (move_player_system, update_map).chain(),
                    (check_chests_system, display_score_gain_system).chain(),
                ),
            );
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            far: 1000.,
            near: -1000.,
            scale: 0.3,
            ..default()
        },
        ..default()
    });
}
