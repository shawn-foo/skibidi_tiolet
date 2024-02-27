use bevy::prelude::*;
use std::fs;

use super::NO_OF_LEVELS;


pub struct Level{
    level: [[i32]],
}

pub fn load_level(
    mut level_storage: LevelStorage
){
    for l in NO_OF_LEVELS.iter(){
        let level_string = fs::read_to_string(format)
        let x: MyStruct = ron::from_str("(boolean: true, float: 1.23)").unwrap();
    
        println!("RON: {}", ron::to_string(&x).unwrap());
    
        println!("Pretty RON: {}", ron::ser::to_string_pretty(
            &x, ron::ser::PrettyConfig::default()).unwrap(),
        );

    }
}

#[derive(Resource)]
pub struct LevelStorage(pub [Level]);
impl Default for LevelStorage{
    fn default() -> Self{
        Self([])
    }
}