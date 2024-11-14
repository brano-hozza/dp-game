pub mod common;
pub mod map;
pub mod player;
pub mod score;
use bevy::prelude::*;

pub mod systems;

use common::systems::print_position_system;
use map::{resources::GameMap, systems::update_map, MapTile};
use player::systems::move_player_system;
use score::{components::GainScoreEvent, systems::display_score_gain_system};
use systems::{check_chests_system, setup};

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
            .add_systems(Startup, setup)
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
