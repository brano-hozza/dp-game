pub mod common;
pub mod map;
pub mod player;
pub mod score;
pub mod ui;

use bevy::prelude::*;

use common::systems::print_position_system;
use map::{resources::GameMap, systems::update_map, MapTile};
use player::systems::move_player_system;
use score::{
    components::GainScoreEvent,
    systems::{check_chests_system, display_score_gain_system},
};

pub struct DpPlugin;

impl Plugin for DpPlugin {
    fn build(&self, app: &mut App) {
        let json_map: Vec<Vec<u32>> =
            serde_json::from_str(include_str!("../assets/maps/01.json")).unwrap();
        let game_map: Vec<Vec<MapTile>> = json_map
            .iter()
            .map(|row| row.iter().map(|tile| MapTile::from(*tile)).collect())
            .collect();
        app.insert_resource(GameMap(game_map))
            .add_event::<GainScoreEvent>()
            .add_systems(
                Startup,
                (
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
