use bevy::{
    app::AppExit, math::Vec3, window::{Cursor, CursorGrabMode}
};
use bevy_infinite_grid::InfiniteGridSettings;

use editor::{
    camera::{self, MainCam},
    csg::{self, Brushable},
    prelude::*,
};
use common::input;

pub fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Spoker Map Editor".to_string(),
                cursor: Cursor {
                    grab_mode: CursorGrabMode::None,
                    visible: true,
                    ..default()
                },
                ..default()
            }),
            ..default()
        }),
        //RapierPhysicsPlugin::<NoUserData>::default(),
        bevy_infinite_grid::InfiniteGridPlugin,
        input::InputPlugin,
        camera::CameraPlugin,
        csg::CsgPlugin,
    ));

    app.add_systems(Startup, setup);
    app.add_systems(Update, (draw_y_axis, quit_on_ctrl_q));

    app.run()
}

pub fn setup(mut commands: Commands) {
    commands.spawn(bevy_infinite_grid::InfiniteGridBundle {
        settings: InfiniteGridSettings {
            x_axis_color: Color::rgb(0.2, 1.0, 0.2),
            z_axis_color: Color::rgb(0.2, 0.2, 1.0),
            ..default()
        },
        ..default()
    });

    commands.spawn((
        Camera3dBundle::default(),
        input::Mouse::default(),
        input::MovAxis3::default(),
        input::dont_update::<input::Mouse>(),
        input::dont_update::<input::MovAxis3>(),
        MainCam,
    ));

    commands.spawn((
        Cuboid::from_size(Vec3::splat(3.0)).to_default_brush(),
    ));
}

pub fn draw_y_axis(mut gizmos: Gizmos) {
    gizmos.line(Vec3::ZERO, Vec3::Y * 1000., Color::rgb(1.0, 0.2, 0.2))
}

pub fn quit_on_ctrl_q(
    mut quit_event: EventWriter<AppExit>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight]) 
    && keys.just_pressed(KeyCode::KeyQ) {
        quit_event.send(AppExit);
    }
}
