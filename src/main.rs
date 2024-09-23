use bevy_fast_tilemap::plugin::FastTileMapPlugin;
use dp_game::DpPlugin;

use bevy::prelude::*;
use bevy::window::PresentMode;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: String::from("Fast Tilemap example"),
                    resolution: (1820., 920.).into(),
                    // disable vsync so we can see the raw FPS speed
                    present_mode: PresentMode::Immediate,
                    ..default()
                }),
                ..default()
            }),
            FastTileMapPlugin::default(),
        ))
        .add_plugins(DpPlugin)
        .run();
}
