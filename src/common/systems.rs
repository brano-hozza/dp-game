use bevy::prelude::*;

use super::components::Position;

pub fn print_position_system(
    query: Query<(&Position, &Name)>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyP) {
        for (position, _) in &query {
            println!("Current: x:{}, y:{}", position.x, position.y);
            println!("Last: x:{}, y:{}", position.last_x, position.last_y);
        }
    }
}
