use bevy::prelude::*;

mod framework;
use framework::FrameworkPlugin;
use bevy::window::{PresentMode, WindowResolution};
use bevy::input::common_conditions::input_toggle_active;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Option::Some(Window {
                title: "Bevy Rust Experiments".to_string(),
                resizable: true,
                present_mode: PresentMode::AutoVsync,
                // canvas: Some("#bevy".to_string()),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape)),
        )
        // .add_plugins((PlayerPlugin, WorldPlugin))
        .add_plugins(FrameworkPlugin)
        .run()
}