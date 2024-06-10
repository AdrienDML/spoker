use bevy_infinite_grid::InfiniteGridSettings;

use common::input;
use editor::{
    camera,
    csg::{self, brush_mesh::BrushMesh, convert::Slope, Brushable, CsgLeaf, CsgOp, CsgRoot},
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

