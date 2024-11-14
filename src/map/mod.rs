pub mod resources;
pub mod systems;

pub const MAP_WIDTH: u32 = 10;
pub const MAP_HEIGHT: u32 = 10;

#[derive(Clone, Copy)]
pub enum MapTile {
    WallCornerTopLeft = 0,
    WallTop = 1,
    WallCornerTopRight = 5,
    WallLeft = 10,
    Empty = 11,
    WallRight = 15,
    DoorDown1 = 36,
    DoorDown2 = 37,
    Trapdoor = 38,
    Ladder = 39,
    WallCornerBottomLeft = 40,
    WallBottom = 41,
    WallCornerBottomRight = 45,
    Chest1 = 80,
    Chest2 = 81,
    Torch = 90,
}

impl Into<u32> for MapTile {
    fn into(self) -> u32 {
        self as u32
    }
}

impl From<u32> for MapTile {
    fn from(value: u32) -> Self {
        match value {
            0 => MapTile::WallCornerTopLeft,
            1 => MapTile::WallTop,
            5 => MapTile::WallCornerTopRight,
            10 => MapTile::WallLeft,
            11 => MapTile::Empty,
            15 => MapTile::WallRight,
            36 => MapTile::DoorDown1,
            37 => MapTile::DoorDown2,
            38 => MapTile::Trapdoor,
            39 => MapTile::Ladder,
            40 => MapTile::WallCornerBottomLeft,
            41 => MapTile::WallBottom,
            45 => MapTile::WallCornerBottomRight,
            80 => MapTile::Chest1,
            81 => MapTile::Chest2,
            90 => MapTile::Torch,
            _ => MapTile::Empty,
        }
    }
}
