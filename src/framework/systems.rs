use bevy::{prelude::*, window::PrimaryWindow};

use super::components::Camera;
use crate::player::{self, components::Player};

pub fn spawn_camera(
    mut commands: Commands,
){
    commands.spawn(
        (
            Camera2dBundle::default(),
            Camera
        )
    );
}

pub fn align_camera_to_player(
    mut camera_query: Query<(&mut Transform), (With<Camera>)>,
    window_query: Query<(&Window), (With<PrimaryWindow>)>,
    player_transform: Query<(&mut Transform), (With<Player>, Without<Camera>)>
){
    if let Ok(window) = window_query.get_single(){
        if let Ok(mut camera_transform) = camera_query.get_single_mut(){
            if let Ok(player_transform) = player_transform.get_single(){
                if player_transform.translation.x > 0.0{
                    camera_transform.translation.x = player_transform.translation.x;
                }
            }
        }
    }
}