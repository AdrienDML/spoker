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
        settings::SettingsPlugin,
        player::PlayerPlugin,
        RapierPhysicsPlugin::<NoUserData>::default(),
        RapierDebugRenderPlugin::default(),
    ))
    .add_systems(Update, (release_cursor_on_esc, draw_gizmos))
    .add_systems(Startup, setup_world);
    app.run();
}
