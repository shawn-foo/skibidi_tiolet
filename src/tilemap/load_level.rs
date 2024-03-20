use bevy::prelude::*;

use serde::{Serialize, Deserialize};

use std::fs;

use crate::console_log;

use super::{levels::get_level, resource::{CurrentLevelInfo, TilemapResource}, tile::TilePosition, NO_OF_LEVELS};

// #[derive(Serialize, Deserialize)]
// pub struct LevelsStruct{
//     pub levels: Vec<Vec<i32>>,
// }
// impl std::fmt::Display for LevelsStruct{
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         write!(f, "levels: /n{:#?}", self.levels)
//     }
// }
// impl LevelsStruct{
//     pub fn empty() -> Self{
//         Self { levels: vec![vec![]] }
//     }
// }



pub fn load_level(
    mut tilemap_resource: ResMut<TilemapResource>,
    mut commands: Commands
){
    let mut player_starting = TilePosition::new(0, 0);
    let level1 = get_level(1);
    let mut found = false;
    for (y, x_vector) in level1.iter().enumerate(){
        for (x, value) in level1.iter().enumerate(){
            if level1[y][x] == 2{

                found = true;
                player_starting = TilePosition::new(x as i32, y as i32);
                break;
            }
        }
        if found == true{
            break;
        }
    }
    let current_level = CurrentLevelInfo{
        level :get_level(1),
        player_starting_pos: player_starting,
        height: level1.len() as i32,
        width: level1[0].len() as i32,
    };
    
    commands.insert_resource(current_level);

    for l in 0..(NO_OF_LEVELS){
        let level = get_level(l+1);
        tilemap_resource.levels.push(level);

    }
    
}
