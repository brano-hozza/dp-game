use crate::common::components::{Name, Position};
use crate::map::resources::GameMap;
use crate::map::{MapTile, MAP_HEIGHT, MAP_WIDTH};
use crate::player::components::Player;
use crate::score::components::{GainScoreEvent, Score};
use bevy::math::{uvec2, vec2};
use bevy::prelude::*;

use bevy_fast_tilemap::prelude::*;

//

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

    let mut root_node = commands.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::SpaceBetween,
            ..default()
        },
        ..default()
    });

    root_node.with_children(|parent| {
        parent.spawn((
            TextBundle::from_section(
                "Score: 0",
                TextStyle {
                    font_size: 30.0,
                    ..default()
                },
            ),
            Label,
            Score(0),
        ));
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
    commands.spawn((
        Position {
            x: 1,
            last_x: 1,
            y: 2,
            last_y: 2,
        },
        Name("Player".to_string()),
        Player,
    ));
}
