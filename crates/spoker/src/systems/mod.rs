use std::f32::consts::PI;

use bevy::{
    prelude::*,
    render::mesh::PlaneMeshBuilder,
    window::{CursorGrabMode, PrimaryWindow},
};
use bevy_rapier3d::prelude::*;

pub mod player;
pub mod debug;

use crate::components::{col_layers, player::SpawnPlayerCmd};

pub fn setup_world(
    mut commands: Commands,
    mut ambient_light: ResMut<AmbientLight>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    ambient_light.brightness = 400.0;
    ambient_light.color = Color::WHITE;
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

    // Temporary Ground.
    commands
        .spawn((
            PbrBundle {
                mesh: ground_mesh,
                material: ground_mat,
                transform: Transform::from_translation(Vec3::ZERO),
                ..default()
            },
            RigidBody::Fixed,
            CollisionGroups {
                memberships: col_layers::ENVIRONEMENT,
                filters: col_layers::PLAYERS,
            },
        ))
        .with_children(|children| {
            children.spawn((
                Collider::cuboid(100.0, 1.0, 100.0),
                TransformBundle::from(Transform::from_translation(-Vec3::Y)),
            ));
        });

    // Light.
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

    // Little green cube.
    commands.spawn(PbrBundle {
        mesh: cube_mesh,
        material: cube_mat,
        transform: Transform::from_translation(Vec3::Y * 0.5).with_rotation(Quat::from_axis_angle(Vec3::Y, PI/6.0)),
        ..default()
    });

    commands.add(SpawnPlayerCmd {
        transform: Transform::default(),
        cam_offset: Vec3::Y,
    });
}



pub fn release_cursor_on_esc(
    esc_input: Res<ButtonInput<KeyCode>>,
    mut window: Query<&mut Window, With<PrimaryWindow>>,
) {
    let mut window = window.single_mut();
    if esc_input.just_pressed(KeyCode::Escape) {
        let cursor = &mut window.cursor;
        match cursor.grab_mode {
            CursorGrabMode::Locked => {
                cursor.grab_mode = CursorGrabMode::None;
                cursor.visible = true;
            }
            _ => {
                cursor.grab_mode = CursorGrabMode::Locked;
                cursor.visible = false;
            }
        }
    }
}
