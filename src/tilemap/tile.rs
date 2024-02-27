use bevy::prelude::*;


#[derive(Component)]
pub struct Tile{
    tile_type: TileType,
    position: TilePosition
}

pub enum TileType{
    WALL
}

pub struct TilePosition{
    x: i32,
    y: i32
}