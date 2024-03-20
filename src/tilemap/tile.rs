use bevy::{prelude::*, render::extract_component::ExtractComponent};

use std::fmt;

#[derive(Component)]
pub struct Tile{
    tile_type: TileType,
    position: TilePosition
}
impl Tile{
    pub fn new(tile_type: TileType, position: TilePosition) -> Self{
        Self { tile_type, position }
    }
}


pub enum TileType{
    WALL
}
#[derive(Clone)]
pub struct TilePosition{
    pub x: i32,
    pub y: i32
}
impl TilePosition{
    pub fn new(x: i32, y: i32) -> Self{
        Self { x, y }
    }
}
impl fmt::Display for TilePosition{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "tileposition: /n x{} y{}", self.x, self.y)
    }
}
#[derive(Component)]
pub struct BottomTile{
    above_tiles: [Entity]
}

