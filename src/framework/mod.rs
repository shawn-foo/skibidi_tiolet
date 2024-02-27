use bevy::prelude::*;

mod components;

mod systems;
use systems::spawn_camera;

pub struct FrameworkPlugin;
impl Plugin for FrameworkPlugin{
    fn build(&self, app: &mut App){
        app
            .add_systems(Startup, spawn_camera)
        ;
    }
}