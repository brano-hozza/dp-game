use crate::components::{Name, Player, Position};
use bevy::prelude::*;

pub fn print_position_system(query: Query<(&Position, &Name)>) {
    for (position, name) in &query {
        println!("{}, {}", name, position);
    }
}

pub fn setup(mut commands: Commands) {
    commands.spawn((
        Position { x: 1.0, y: 2.0 },
        Name("Player".to_string()),
        Player,
    ));
}
