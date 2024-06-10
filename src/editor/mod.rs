use crate::prelude::*;
use crate::AppState;

pub mod camera;
pub mod csg;
pub mod r#move;
pub mod select;
pub mod ui;

use csg::{brush_mesh::BrushMesh, convert::Slope, Brushable, CsgLeaf, CsgOp, CsgRoot};
use ui::MouseOnMap;

#[derive(SystemSet, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct RunOnMapFocused;

#[derive(SystemSet, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct EditorSet;

pub struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        {
            app.configure_sets(Update, EditorSet.run_if(in_state(AppState::Editor)))
                .configure_sets(
                    Update,
                    RunOnMapFocused
                        .run_if(resource_equals(MouseOnMap(true)))
                        .in_set(EditorSet),
                )
                .add_systems(OnEnter(AppState::Editor), setup.after(ui::setup))
        };
    }
}

pub fn setup(
    mut commands: Commands,
    mut brushes: ResMut<Assets<BrushMesh>>,
    mut ambient_light: ResMut<AmbientLight>,
) {
    ambient_light.brightness = 400.0;
    ambient_light.color = Color::WHITE;
    let mut light_dir = Transform::default();
    light_dir.look_to(Vec3::NEG_Y + Vec3::X, Vec3::Y);
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::WHITE,
            illuminance: 1000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: light_dir,
        ..default()
    });

    commands.spawn(bevy_infinite_grid::InfiniteGridBundle {
        settings: InfiniteGridSettings {
            x_axis_color: Color::rgb(0.2, 1.0, 0.2),
            z_axis_color: Color::rgb(0.2, 0.2, 1.0),
            ..default()
        },
        ..default()
    });

    commands
        .spawn((CsgRoot, TransformBundle::default()))
        .with_children(|root| {
            root.spawn((
                Name::new("Cube"),
                CsgLeaf,
                CsgOp::Add,
                brushes.add(Cuboid::from_size(Vec3::splat(1.0)).to_default_brush()),
                TransformBundle {
                    local: Transform::from_translation(0.5 * Vec3::Y),
                    ..default()
                },
            ));

            root.spawn((
                Name::new("Slope"),
                CsgLeaf,
                CsgOp::Add,
                brushes.add(Slope {
                    length: 2.0,
                    height: 1.0,
                    width: 1.0,
                }),
                TransformBundle {
                    local: Transform::from_translation(Vec3::Z),
                    ..default()
                },
            ));
        });
}

pub fn draw_y_axis(mut gizmos: Gizmos) {
    gizmos.line(Vec3::ZERO, Vec3::Y * 1000., Color::rgb(1.0, 0.2, 0.2))
}
