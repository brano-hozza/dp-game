use bevy::prelude::*;

use crate::{
    common::components::{Name, Position},
    map::{MAP_HEIGHT, MAP_WIDTH},
};

use super::components::Player;

pub fn move_player_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Position, &Player)>,
) {
    for (mut position, _player) in query.iter_mut() {
        position.last_x = position.x;
        position.last_y = position.y;
        if keyboard_input.just_pressed(KeyCode::ArrowLeft) && position.x > 1 {
            position.x -= 1;
        }
        if keyboard_input.just_pressed(KeyCode::ArrowRight) && position.x < MAP_WIDTH - 2 {
            position.x += 1;
        }
        if keyboard_input.just_pressed(KeyCode::ArrowUp) && position.y > 1 {
            position.y -= 1;
        }
        if keyboard_input.just_pressed(KeyCode::ArrowDown) && position.y < MAP_HEIGHT - 2 {
            position.y += 1;
        }
    }
}

//

pub fn setup(mut commands: Commands) {
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
