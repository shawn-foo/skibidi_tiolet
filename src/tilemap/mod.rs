use bevy::prelude::*;

pub mod tile;

pub mod resource;
use resource::TilemapResource;

mod load_level;
use load_level::load_level;

mod systems;
use systems::spawn_tilemap;

use crate::utils::Size;

use self::resource::CurrentLevelInfo;

mod levels;



//constants
pub const NO_OF_LEVELS: i32 = 1;
pub const PADDING_BEFORE_DESPAWN: f32 = 200.0;
pub const TILE_SIZE: Size = Size{
    height: 50.0,
    width: 50.0
};

pub struct TileMapPlugin;
impl Plugin for TileMapPlugin{
    fn build(&self,  app: &mut App){
        app
            .add_systems(PreStartup, load_level)
            .add_systems(Startup, spawn_tilemap)
            .init_resource::<TilemapResource>()
        ;
    }
}