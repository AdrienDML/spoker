use bevy::{prelude::*, window::Cursor};
#[cfg(not(feature = "reload"))]
use systems::*;

#[cfg(feature = "reload")]
use systems_hot::*;

#[cfg(feature = "reload")]
#[hot_lib_reloader::hot_module(dylib = "systems")]
mod systems_hot {
    pub use components::*;
    pub use systems::*;
    hot_functions_from_file!("systems/src/lib.rs");
}

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Me Float".to_string(),
            cursor: Cursor {
                grab_mode: bevy::window::CursorGrabMode::Locked,
                visible: false,
                ..default()
            },
            ..default()
        }),
        ..default()
    }), SettingsPlugin, PlayerPlugin))
        .add_systems(Startup, setup_world);
    app.run();
}
