use bevy::prelude::*;

#[derive(Component)]
pub struct Player{
    pub on_floor: bool,
    pub vel_y: f32,
    pub vel_x: f32,
    pub jumping: bool,
}
impl Player{
    pub fn new(on_floor: bool, vel_y: f32, vel_x: f32, jumping: bool) -> Self{
        Self { 
            on_floor,
            vel_y,
            vel_x,
            jumping
        }
    }
}