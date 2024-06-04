use bevy::window::{Cursor, CursorGrabMode};

use spoker::*;
use prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Me Float".to_string(),
                cursor: Cursor {
                    grab_mode: CursorGrabMode::Locked,
                    visible: false,
                    ..default()
                },
                ..default()
            }),
            ..default()
        }).set(bevy::log::LogPlugin {
                filter:"player::movement=info,common::physics=info,component::player=info".to_string(),
                ..default()
            }),
        Aery,
        common::input::InputPlugin,
        common::physics::PhysicsPlugin,
    ))
    .add_plugins((
            environement::EnvironementPlugin,
            player::PlayerPlugin,
            ui::UiPlugin,
    ));



    #[cfg(debug_assertions)]
    app.add_plugins(spoker::debug::DebugPlugin);

    app.run();
}

