use bevy::prelude::*;

pub mod components;

mod systems;
use systems::{spawn_player, move_player};

use crate::utils::Size;

//constants
pub const PLAYER_SIZE: Size = Size{
    height: 80.0,
    width: 40.0
};
pub const GRAVITY: f32 = -1.0;
pub const JUMP_STRENGTH: f32 = 20.0;
pub const HORISONTAL_MOVEMENT_SPEED: f32 = 5.0;
pub const GROUND_FRICTION: f32 = 1.0;


pub struct PlayerPlugin;
impl Plugin for PlayerPlugin{
    fn build(&self, app: &mut App){
        app
            .add_systems(Startup, spawn_player)
            .add_systems(Update, (move_player))
        ;
    }
}