use std::{str::FromStr, sync::Arc};

use alloy::{
    primitives::{address, Address},
    providers::ProviderBuilder,
    sol_types::sol,
};
use bevy::prelude::*;
use bevy_tokio_tasks::TokioTasksRuntime;
use common::systems::print_position_system;
use map::{resources::GameMap, systems::update_map, MapTile};
use player::systems::move_player_system;
use score::{
    components::GainScoreEvent,
    systems::{check_chests_system, display_score_gain_system},
};
use tokio::sync::RwLock;
// alloy - alternative to web3

pub mod common;
pub mod map;
pub mod player;
pub mod score;
pub mod ui;

const TEST_1_ADDRESS: &str = "0xbFc4FDBfb53A129a3E78f4291D5Dd7d00045E18B";
const TEST_1_PK: &str = "cec396dab69ee79d9ef52f1bf4d3ce98e110c33318a85dea5fc7fc93feda7480";

const TEST_3_ADDRESS: &str = "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266";

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    GoldContract,
    "assets/contracts/Gold.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    SwordContract,
    "assets/contracts/Sword.json"
);

fn display_balances(runtime: Res<TokioTasksRuntime>) {
    runtime.spawn_background_task(|_ctx| {
        let rpc_url = "127.0.0.1:8545";

        // Create the provider and wrap it in Arc to share ownership across async blocks.
        let provider = Arc::new(
            ProviderBuilder::new().on_anvil_with_wallet_and_config(|anvil| anvil.fork(rpc_url)),
        );

        // Create the gold_contract and wrap it in Arc<Mutex> to allow thread-safe access.
        let gold_contract = Arc::new(RwLock::new(GoldContract::new(
            address!("5FbDB2315678afecb367f032d93F642f64180aa3"),
            provider.clone(), // Clone the Arc to share ownership of the provider.
        )));

        // Move everything into the async block, ensuring all variables are owned.
        async move {
            let accounts: Vec<Address> = vec![
                Address::from_str(TEST_1_ADDRESS).unwrap(),
                Address::from_str(TEST_3_ADDRESS).unwrap(),
            ];

            for account in accounts {
                // Lock the Mutex to safely access the gold_contract.
                let contract = gold_contract.read().await;

                // Use the contract to make async calls.
                let res = contract.balanceOf(account).await.unwrap();
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
        app.add_plugins(bevy_tokio_tasks::TokioTasksPlugin::default())
            .insert_resource(GameMap(game_map))
            .add_event::<GainScoreEvent>()
            .add_systems(
                Startup,
                (
                    display_balances,
                    setup,
                    ui::systems::setup,
                    map::systems::setup,
                    player::systems::setup,
                ),
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
