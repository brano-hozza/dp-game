use bevy::prelude::*;
use core::fmt;

#[derive(Component)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Position: ({}, {})", self.x, self.y)
    }
}

#[derive(Component)]
pub struct Name(pub String);

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Name: {}", self.0)
    }
}

#[derive(Component)]
pub struct Player;
