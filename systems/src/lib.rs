#![allow(clippy::type_complexity)]
use bevy::{prelude::*, render::mesh::PlaneMeshBuilder};

mod player;
mod settings;

use components::SpawnPlayerCmd;
pub use player::*;
pub use settings::*;

pub fn setup_world(
    mut commands: Commands,
    mut ambient_light: ResMut<AmbientLight>,
    mut clear_color: ResMut<ClearColor>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    ambient_light.brightness = 10.0;
    clear_color.0 = Color::RED;
    let ground_mesh = meshes.add(PlaneMeshBuilder {
        half_size: Vec2::splat(100.0),
        plane: Plane3d::new(Vec3::Y),
    });
    let ground_mat = materials.add(StandardMaterial { ..default() });
    let cube_mesh = meshes.add(Cuboid {
        half_size: Vec3::splat(0.5),
    });
    let cube_mat = materials.add(StandardMaterial {
        base_color: Color::GREEN,
        ..default()
    });
    commands.spawn(PbrBundle {
        mesh: ground_mesh,
        material: ground_mat,
        transform: Transform::from_translation(Vec3::ZERO),
        ..default()
    });
    commands.spawn(PbrBundle {
        mesh: cube_mesh,
        material: cube_mat,
        transform: Transform::from_translation(Vec3::Y * 0.5),
        ..default()
    });
    commands.add(SpawnPlayerCmd {
        transform: Transform::default(),
        cam_offset: Vec3::Y,
    })
}
