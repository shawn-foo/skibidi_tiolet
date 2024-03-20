use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;

use bevy::math::bounding::Aabb2d;
use bevy::math::vec2;


mod utils;
use crate::utils::check_collision;

pub mod framework;
use framework::FrameworkPlugin;

pub mod player;
use player::PlayerPlugin;

mod tilemap;
use tilemap::TileMapPlugin;

use bevy::window::{PresentMode, WindowResolution};
use bevy::input::common_conditions::input_toggle_active;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    
    App::new()
    .insert_resource(AssetMetaCheck::Never)
        .add_plugins(
            DefaultPlugins.set(
            (WindowPlugin {
            primary_window: Option::Some(Window {
                title: "Bevy Rust Experiments".to_string(),
                resizable: true,
                present_mode: PresentMode::AutoVsync,
                // canvas: Some("#bevy".to_string()),
                ..default()
            }),
            ..default()
        })
    ))

        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape)),
        )
        // .add_plugins((PlayerPlugin, WorldPlugin))
        .add_plugins((FrameworkPlugin, TileMapPlugin, PlayerPlugin))
        .run()
}