use std::{str::FromStr, sync::Arc};

use alloy::{
    network::{Ethereum, EthereumWallet},
    primitives::{address, Address, Uint},
    providers::{
        fillers::{
            BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller,
            WalletFiller,
        },
        Identity, Provider, ProviderBuilder, RootProvider,
    },
    signers::local::PrivateKeySigner,
    sol_types::sol,
    transports::http::{Client, Http},
};
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

pub mod common;
pub mod map;
pub mod player;
pub mod score;
pub mod ui;

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
type ContractProvider = Arc<
    FillProvider<
        JoinFill<
            JoinFill<
                Identity,
                JoinFill<GasFiller, JoinFill<BlobGasFiller, JoinFill<NonceFiller, ChainIdFiller>>>,
            >,
            WalletFiller<EthereumWallet>,
        >,
        RootProvider<Http<Client>>,
        Http<Client>,
        Ethereum,
    >,
>;

#[derive(Resource)]
pub struct EthProviderResource(ContractProvider);

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
    let sword_contract = sword_contract_res.0.clone();
    let provider = provider.0.clone();
    runtime.spawn_background_task(|_ctx| {
        // Move everything into the async block, ensuring all variables are owned.
        async move {
            let account_alice = Address::from_str(ALICE_ADDRESS).unwrap();
            let nonce = provider
                .get_transaction_count(account_alice)
                .await
                .expect("Failed to get nonce");

            let tx_hash = sword_contract
                .mint(account_alice, Uint::from(3))
                .nonce(nonce + 1)
                .send()
                .await
                .expect("Failed to mint sword")
                .watch()
                .await
                .expect("Failed to watch");

            println!("Minted sword with tx hash: {}", tx_hash);

            // Transfer sword from Alice to Bob

            let account_bob = Address::from_str(BOB_ADDRESS).unwrap();

            let tx_hash = sword_contract
                .transferFrom(account_alice, account_bob, Uint::from(3))
                .send()
                .await
                .expect("Failed to transfer sword")
                .watch()
                .await
                .expect("Failed to watch");

            println!("Transferred sword with tx hash: {}", tx_hash);
        }
    });
}

pub fn listen_to_gold_transfers(
    runtime: Res<TokioTasksRuntime>,
    gold_contract_res: Res<GoldContractResource>,
) {
    let gold_contract = gold_contract_res.0.clone();
    runtime.spawn_background_task(|_ctx| {
        // Move everything into the async block, ensuring all variables are owned.
        async move {
            let gold_filter = gold_contract
                .Transfer_filter()
                .watch()
                .await
                .expect("Failed to watch");

            gold_filter
                .into_stream()
                .take(1)
                .for_each(|log| {
                    match log {
                        Ok((_event, log)) => {
                            println!("Received Gold Transfer: {log:?}");
                        }
                        Err(e) => {
                            println!("Log event error: {e:?}");
                        }
                    };
                })
                .await;
        }
    });
}

pub fn listen_to_sword_transfers(
    runtime: Res<TokioTasksRuntime>,
    sword_contract_res: Res<SwordContractResource>,
) {
    let sword_contract = sword_contract_res.0.clone();
    runtime.spawn_background_task(|_ctx| {
        // Move everything into the async block, ensuring all variables are owned.
        async move {
            let transfer_filter = sword_contract
                .Transfer_filter()
                .watch()
                .await
                .expect("Failed to watch");

            transfer_filter
                .into_stream()
                .take(1)
                .for_each(|log| {
                    match log {
                        Ok((_event, log)) => {
                            println!("Received Sword Transfer: {log:?}");
                        }
                        Err(e) => {
                            println!("Log event error: {e:?}");
                        }
                    };
                })
                .await;
        }
    });
}

pub fn try_gold_transfer(
    runtime: Res<TokioTasksRuntime>,
    gold_contract_res: Res<GoldContractResource>,
) {
    let gold_contract = gold_contract_res.0.clone();
    runtime.spawn_background_task(|_ctx| {
        // Move everything into the async block, ensuring all variables are owned.
        async move {
            let account_bob = Address::from_str(BOB_ADDRESS).unwrap();

            let tx = gold_contract
                .transfer(account_bob, Uint::from(1))
                .send()
                .await
                .expect("Failed to transfer gold")
                .watch()
                .await
                .expect("Failed to watch");

            println!("Transferred gold with tx hash: {}", tx);

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

        let signer: PrivateKeySigner = ALICE_PK.parse().expect("should parse private key");
        let wallet = EthereumWallet::from(signer);

        // Create the provider and wrap it in Arc to share ownership across async blocks.
        let provider = Arc::new(
            ProviderBuilder::new()
                .with_recommended_fillers()
                .wallet(wallet)
                .on_http(rpc_url.parse().expect("Failed to parse rpc url")),
        );

        // Create the gold_contract and wrap it in Arc to allow thread-safe access.
        let gold_contract = GoldContract::new(
            address!("5FbDB2315678afecb367f032d93F642f64180aa3"),
            provider.clone(), // Clone the Arc to share ownership of the provider.
        );

        // Create the sword_contract and wrap it in Arc to allow thread-safe access.
        let sword_contract = SwordContract::new(
            address!("e7f1725E7734CE288F8367e1Bb143E90bb3F0512"),
            provider.clone(), // Clone the Arc to share ownership of the provider.
        );

        app.add_plugins(bevy_tokio_tasks::TokioTasksPlugin::default())
            .insert_resource(GameMap(game_map))
            .insert_resource(EthProviderResource(provider))
            .insert_resource(GoldContractResource(Arc::new(gold_contract)))
            .insert_resource(SwordContractResource(Arc::new(sword_contract)))
            .add_event::<GainScoreEvent>()
            .add_systems(
                Startup,
                (
                    display_balances,
                    listen_to_gold_transfers,
                    // listen_to_sword_transfers,
                    try_gold_transfer,
                    // try_mint_sword,
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
