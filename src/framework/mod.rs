use bevy::prelude::*;

mod components;

mod systems;
use systems::{spawn_camera, align_camera_to_player};

pub struct FrameworkPlugin;
impl Plugin for FrameworkPlugin{
    fn build(&self, app: &mut App){
        app
            .add_systems(Startup, spawn_camera)
            .add_systems(Update, align_camera_to_player)
        ;
    }
}