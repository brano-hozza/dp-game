use bevy::prelude::*;

use super::MapTile;

#[derive(Resource)]
pub struct GameMap(pub Vec<Vec<MapTile>>);

impl GameMap {
    pub fn get(&self, x: usize, y: usize) -> MapTile {
        self.0[y][x]
    }

    pub fn clear_tile(&mut self, x: usize, y: usize) {
        self.0[y][x] = MapTile::Empty;
    }
}
