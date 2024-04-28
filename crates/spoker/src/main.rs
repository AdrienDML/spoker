use bevy::window::Cursor;
use bevy::window::CursorGrabMode;
use spoker::prelude::*;
use spoker::systems::*;

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
        }),
        Aery,
        common::input::InputPlugin,
        RapierPhysicsPlugin::<NoUserData>::default(),
    ))
    .add_plugins(player::PlayerPlugin)
    .add_systems(Update, release_cursor_on_esc)
    .add_systems(Startup, setup_world);

    #[cfg(debug_assertions)]
    app.add_plugins(debug::DebugPlugin);

    app.run();
}
