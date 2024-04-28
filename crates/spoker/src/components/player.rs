use crate::{components::col_layers, prelude::*};
use common::input;

use bevy::ecs::system::Command;

#[derive(Component)]
pub struct Player;

// No clip speed.
#[derive(Component)]
pub struct NoClip(pub f32);

// No clip speed.
#[derive(Component)]
pub struct FlyCam {
    pub speed: f32,
    pub return_pos: Transform,
}

#[derive(Component, Deref, DerefMut)]
pub struct FlyCamLastPos(Option<Transform>);

// Update the player velocity variable and set jump and dash flags.
#[derive(Component)]
pub struct PlayerCam;

pub struct SpawnPlayerCmd {
    pub transform: Transform,
    pub cam_offset: Vec3,
}

impl Command for SpawnPlayerCmd {
    fn apply(self, world: &mut World) {

        let player_mesh = world
            .get_resource_mut::<Assets<Mesh>>()
            .unwrap()
            .add(Capsule3d::new(0.5, 1.0).mesh());

        let player_mat = world
            .get_resource_mut::<Assets<StandardMaterial>>()
            .unwrap()
            .add(StandardMaterial::default());

        world
            .spawn((
                PbrBundle {
                    mesh: player_mesh,
                    material: player_mat,
                    transform: self.transform,
                    ..default()
                },
                Player,
                (
                    input::Mouse::default(),
                    input::MovAxis3::default(),
                ),
                // Physics Related.
                (
                    RigidBody::Dynamic,
                    Velocity {
                        linvel: Vec3::ZERO,
                        angvel: Vec3::ZERO,
                    },
                    Sleeping::disabled(),
                    Ccd::enabled(),
                    GravityScale(1.0),
                    LockedAxes::ROTATION_LOCKED,
                    Collider::capsule_y(0.5, 0.5),
                    ColliderMassProperties::Mass(1.0),
                    CollisionGroups {
                        memberships: col_layers::PLAYERS | col_layers::HURTBOXES,
                        filters: col_layers::ENVIRONEMENT
                            | col_layers::HITBOXES
                            | col_layers::PLAYERS,
                    },
                    ExternalImpulse::default(),
                ),
            ))
            .with_children(|cb| {
                cb.spawn_with_rel((
                    Camera3dBundle {
                        transform: Transform::from_translation(self.cam_offset),
                        ..default()
                    },
                    FlyCamLastPos(None),
                    PlayerCam,
                ));
            });
    }
}
