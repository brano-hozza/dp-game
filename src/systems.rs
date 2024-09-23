use crate::components::{Name, Player, Position};
use bevy::math::{uvec2, vec2};
use bevy::prelude::*;

use bevy_fast_tilemap::prelude::*;

pub fn print_position_system(query: Query<(&Position, &Name)>) {
    for (position, name) in &query {
        println!("{}, {}", name, position);
    }
}

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<Map>>,
) {
    commands.spawn(Camera2dBundle::default());

    let json_map: Vec<Vec<u32>> =
        serde_json::from_str(include_str!("../assets/maps/01.json")).unwrap();

    let map = Map::builder(
        // Map size
        uvec2(10, 10),
        // Tile atlas
        asset_server.load("tilesets/dungeon.png"),
        // Tile Size
        vec2(16., 16.),
    )
    .build_and_initialize(|map| {
        for (y, row) in json_map.iter().enumerate() {
            for (x, &tile) in row.iter().enumerate() {
                map.set(x as u32, y as u32, tile);
            }
        }
    });

    commands.spawn(MapBundleManaged::new(map, materials.as_mut()));
    commands.spawn((
        Position { x: 1.0, y: 2.0 },
        Name("Player".to_string()),
        Player,
    ));
}
