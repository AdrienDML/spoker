use crate::{components::col_layers, settings::*, prelude::*};
use bevy::ecs::system::Command;

#[derive(Component)]
pub struct Player;

// No clip speed.
#[derive(Component)]
pub struct NoClip(pub f32);

// No clip speed.
#[derive(Component)]
pub struct Inputs {
    pub dir: Vec3,
    pub jump: bool,
    pub crouch: bool,
    pub mouse: Vec2,
}

// No clip speed.
#[derive(Component)]
pub struct FlyCam {
    pub speed: f32,
    pub return_pos: Transform,
}

#[derive(Reflect, Actionlike, Clone, Eq, PartialEq, Hash)]
pub enum Movement {
    Horizontal,
    Up,
    Down,
}

#[derive(Reflect, Actionlike, Clone, Eq, PartialEq, Hash)]
pub struct Mouse {}

#[derive(Reflect, Actionlike, Clone, Eq, PartialEq, Hash)]
pub enum PlayerControl {
    ToggleNoclip,
    ToggleFlyCam,
}

#[derive(Reflect, Actionlike, Clone, Eq, PartialEq, Hash)]
pub enum FlyAction {
    SwitchMode,
    IncSpeed,
    DecSpeed,
}

#[derive(Reflect, Actionlike, Clone, Eq, PartialEq, Hash)]
pub enum State {
    Grounded,
    InAir,
}

//#[derive(Reflect, Actionlike, Clone, Eq, PartialEq, Hash)]
//pub enum PlayerAction {
//    Scope,
//    Dash,
//}

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
        let movement_input_map = world
            .get_resource::<movement::MovementSettings>()
            .expect("No movement settings setup.")
            .into();
        let control_input_map = world
            .get_resource::<player::ControlSettings>()
            .expect("No control settings setup.")
            .into();

        let fly_input_map = world
            .get_resource::<movement::FlySettings>()
            .expect("No control settings setup.")
            .into();

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
                // Inputs
                (
                    InputManagerBundle::with_map(movement_input_map),
                    InputManagerBundle::with_map(InputMap::new([(Mouse {}, DualAxis::mouse_motion())])),
                    InputManagerBundle::with_map(control_input_map),
                    InputManagerBundle::with_map(fly_input_map),
                    Inputs {
                        dir: Vec3::ZERO,
                        jump: false,
                        crouch: false,
                        mouse: Vec2::ZERO,
                    }
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
                        filters: col_layers::ENVIRONEMENT | col_layers::HITBOXES | col_layers::PLAYERS,
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
