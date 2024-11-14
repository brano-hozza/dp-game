use bevy::prelude::*;

#[derive(Component)]
pub struct Score(pub u32);

#[derive(Event)]
pub struct GainScoreEvent(pub u32);
