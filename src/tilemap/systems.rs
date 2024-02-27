use bevy::prelude::*;

use super::components::Camera;

pub fn spawn_camera(
    mut commands: Commands,
){
    commands.spawn(
        (
            Camera2dBundle::default(),

        )
    )
}