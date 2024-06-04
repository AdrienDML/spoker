use bevy_infinite_grid::InfiniteGridSettings;

use common::input;
use editor::{
    camera,
    csg::{self, brush_mesh::BrushMesh, Brushable, CsgLeaf},
    prelude::*,
    ui::{self, MouseOnMap},
    RunOnMapFocused,
};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Sickle UI -  Simple Editor".into(),
                    resolution: (1280., 720.).into(),
                    ..default()
                }),
                ..default()
            }),
            bevy_infinite_grid::InfiniteGridPlugin,
            input::InputPlugin,
            camera::CameraPlugin,
            csg::CsgPlugin,
        ))
        .configure_sets(
            Update,
            RunOnMapFocused.run_if(resource_equals(MouseOnMap(true))),
        )
        .add_systems(Startup, setup.after(ui::setup))
        .add_systems(Update, draw_y_axis)
        .add_plugins(ui::UiPlugin)
        .run();
}

pub fn setup(mut commands: Commands, mut brushes: ResMut<Assets<BrushMesh>>) {
    commands.spawn(bevy_infinite_grid::InfiniteGridBundle {
        settings: InfiniteGridSettings {
            x_axis_color: Color::rgb(0.2, 1.0, 0.2),
            z_axis_color: Color::rgb(0.2, 0.2, 1.0),
            ..default()
        },
        ..default()
    });

    commands.spawn((
        Name::new("Cube"),
        CsgLeaf,
        CsgOp::Add,
        brushes.add(Cuboid::from_size(Vec3::splat(1.0)).to_default_brush()),
        TransformBundle {
            local: Transform::from_translation(0.5 * Vec3::Y),
            ..default()
        },
    ));
}

pub fn draw_y_axis(mut gizmos: Gizmos) {
    gizmos.line(Vec3::ZERO, Vec3::Y * 1000., Color::rgb(1.0, 0.2, 0.2))
}
