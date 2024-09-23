use bevy::prelude::*;

pub mod components;
pub mod systems;

use systems::{print_position_system, setup};

pub struct DpPlugin;

impl Plugin for DpPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, print_position_system);
    }
}
