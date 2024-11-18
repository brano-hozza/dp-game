use bevy::{
    math::{uvec2, vec2},
    prelude::*,
};
use bevy_fast_tilemap::{bundle::MapBundleManaged, map::Map};

use crate::common::components::Position;

use super::{resources::GameMap, MapTile, MAP_HEIGHT, MAP_WIDTH};

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

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<Map>>,
    game_map: Res<GameMap>,
) {
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            far: 1000.,
            near: -1000.,
            scale: 0.3,
            ..default()
        },
        ..default()
    });

    let map = Map::builder(
        // Map size
        uvec2(MAP_WIDTH, MAP_HEIGHT),
        // Tile atlas
        asset_server.load("tilesets/dungeon.png"),
        // Tile Size
        vec2(16., 16.),
    )
    .build_and_initialize(|map| {
        for (y, row) in game_map.0.iter().enumerate() {
            for (x, &tile) in row.iter().enumerate() {
                map.set(x as u32, y as u32, tile.into());
            }
        }
    });
    commands.spawn(MapBundleManaged::new(map, materials.as_mut()));
}
