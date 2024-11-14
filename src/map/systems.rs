use bevy::prelude::*;
use bevy_fast_tilemap::map::Map;

use crate::common::components::Position;

use super::{resources::GameMap, MapTile};

pub fn update_map(
    mut materials: ResMut<Assets<Map>>,
    maps: Query<&Handle<Map>>,
    position_query: Query<&Position>,
    game_map: Res<GameMap>,
) {
    for map in maps.iter() {
        let map = materials.get_mut(map).unwrap();
        let mut m = map.indexer_mut();
        for pos in position_query.iter() {
            m.set(pos.x, pos.y, MapTile::Torch.into());
            if pos.x == pos.last_x && pos.y == pos.last_y {
                continue;
            }
            m.set(
                pos.last_x,
                pos.last_y,
                game_map
                    .get(pos.last_x as usize, pos.last_y as usize)
                    .into(),
            );
        }
    }
}
