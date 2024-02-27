use bevy::prelude::*;

mod components;

mod systems;
use systems::setup;

pub mod resource;
use resource::TileResource;

mod load_ron;

//constants
pub const NO_OF_LEVELS: i32 = 5;

pub struct FrameworkPlugin;
impl Plugin for FrameworkPlugin{
    fn build(&self, &mut app: App){
        app
            .add_systems(Startup, spawn_camera)
            .init_resource::<TileResource>()
        ;
    }
}