use std::fmt;

use bevy::prelude::*;

#[derive(Component)]
pub struct Position {
    pub x: u32,
    pub last_x: u32,
    pub y: u32,
    pub last_y: u32,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Position: ({}, {})\nLast Position: ({}, {})\n",
            self.x, self.y, self.last_x, self.last_y
        )
    }
}

#[derive(Component)]
pub struct Name(pub String);

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Name: {}", self.0)
    }
}
