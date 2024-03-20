use bevy::{prelude::*};

use crate::utils::Size;

use super::tile::TilePosition;

#[derive(Resource)]
pub struct TilemapResource{
    pub number_in_screen: Size,
//vector of levels(Vec<Vec<i32>>)
    pub levels: Vec<Vec<Vec<i32>>>,
    pub selected_level: i32
}
impl Default for TilemapResource{
    fn default() -> Self {
        Self{
            number_in_screen: Size:: new(10.0, 10.0),
            levels: vec![],
            selected_level: 1
        }
    }
}

#[derive(Resource)]
pub struct CurrentLevelInfo{
    pub level: Vec<Vec<i32>>,
    pub player_starting_pos: TilePosition,
    pub height: i32,
    pub width: i32
}