use bevy::prelude::*;
use bevy_fast_tilemap::map::Map;

use crate::{
    common::components::Position,
    map::{resources::GameMap, MapTile},
};

use super::components::{GainScoreEvent, Score};

pub fn display_score_gain_system(
    mut ev_gain_score: EventReader<GainScoreEvent>,
    mut query: Query<(&mut Score, &mut Text)>,
) {
    for GainScoreEvent(amount) in ev_gain_score.read() {
        for (mut score, mut text) in query.iter_mut() {
            score.0 += amount;
            text.sections[0].value = format!("Score: {} (Last gain {})", score.0, amount);
        }
    }
}

pub fn check_chests_system(
    mut materials: ResMut<Assets<Map>>,
    maps: Query<&Handle<Map>>,
    position_query: Query<&Position>,
    mut game_map: ResMut<GameMap>,
    mut score: Query<&mut Score>,
    mut ev_gain_score: EventWriter<GainScoreEvent>,
) {
    let mut score = score.single_mut();
    for map in maps.iter() {
        let map = materials.get_mut(map).unwrap();
        let mut m = map.indexer_mut();
        for pos in position_query.iter() {
            let tile = game_map.get(pos.x as usize, pos.y as usize);
            match tile {
                MapTile::Chest1 => {
                    game_map.clear_tile(pos.x as usize, pos.y as usize);
                    m.set(pos.x, pos.y, MapTile::Empty.into());
                    score.0 += 1;
                    ev_gain_score.send(GainScoreEvent(1));
                }
                MapTile::Chest2 => {
                    game_map.clear_tile(pos.x as usize, pos.y as usize);
                    m.set(pos.x, pos.y, MapTile::Empty.into());
                    score.0 += 2;
                    ev_gain_score.send(GainScoreEvent(2));
                }
                _ => {}
            }
        }
    }
}
