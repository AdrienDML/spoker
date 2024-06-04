use crate::prelude::*;
use common::input;
use common::physics::{col_layers, prelude::*};

use bevy::ecs::system::Command;

#[derive(Component)]
pub struct Player;

#[derive(Resource, Component)]
pub struct PlayerJumpSettings {
    pub jump_height: f32,
    pub jump_duration: f32,
}

impl Default for PlayerJumpSettings {
    fn default() -> Self {
        Self {
            jump_height: 1.0,
            jump_duration: 0.5,
        }
    }
}

#[derive(Resource, Component)]
pub struct PlayerJumpProperties {
    pub gravity: f32,
    pub jump_speed: f32
}

impl Setting for PlayerJumpSettings {
    type Property = PlayerJumpProperties;

    fn setting(&self) -> Self::Property {
        Self::Property {
            gravity: 4.0 * self.jump_height / (self.jump_duration * self.jump_duration),
            jump_speed: 4.0 * self.jump_height / self.jump_duration,
        }
    }
}

#[derive(Resource, Component)]
pub struct PlayerGroundSettings {
    pub max_speed: f32,
    pub min_speed: f32,
    pub time_to_max_speed: f32,
    pub time_to_stop: f32,
    pub max_slope_traction_angle: f32,
}

impl Default for PlayerGroundSettings {
    fn default() -> Self {
        Self {
            max_speed: 32.0,
            min_speed: 0.5,
            time_to_max_speed: 0.1,
            time_to_stop: 0.2,
            max_slope_traction_angle: PI / 3.0,
        }
    }
}

#[derive(Resource, Component)]
pub struct PlayerGroundProperties {
    pub max_speed: f32,
    pub min_speed: f32,
    pub acceleration_coef: f32,
    pub friction_coef: f32,
    pub max_slope_traction_angle: f32,
}

impl Setting for PlayerGroundSettings {
    type Property = PlayerGroundProperties;

    fn setting(&self) -> Self::Property {
        Self::Property {
            acceleration_coef: f32::ln(self.max_speed / self.min_speed) / self.time_to_max_speed,
            friction_coef: f32::ln(self.max_speed / self.min_speed) / self.time_to_stop,
            max_slope_traction_angle: self.max_slope_traction_angle,
            max_speed: self.max_speed,
            min_speed: self.min_speed,
        }
    }
}


#[derive(Debug, Component)]
pub enum PlayerState {
    Grounded {
        normal: Vec3,
        speed_at_landing: f32,
        tick_since_landing: u32,
    },
    Falling {
        can_coyotee: bool,
        tick_since_falling: u32,
    },
}

impl Default for PlayerState {
    fn default() -> Self {
        Self::Falling {
            tick_since_falling: 0,
            can_coyotee: false,
        }
    }
}

impl PlayerState {
    // Number of frame for lossless
    const MAX_TICK_LOSSLESS_JUMP: u32 = 7;
    // Number of frame where the jump is allowed after falling
    // from a platform.
    const MAX_TICK_COYOTE_JUMP: u32 = 5;

    pub fn set_grounded(&mut self, velocity: &Velocity, normal: Vec3) {
        match self {
            PlayerState::Grounded {
                tick_since_landing, ..
            } => {
                *tick_since_landing = tick_since_landing.saturating_add(1);
            }
            PlayerState::Falling { .. } => {
                *self = PlayerState::Grounded {
                    normal,
                    speed_at_landing: velocity.null_y().length(),
                    tick_since_landing: 0,
                };
            }
        }
    }

    pub fn set_falling(&mut self, can_coyotee: bool) {
        match self {
            PlayerState::Grounded { .. } => {
                *self = PlayerState::Falling {
                    tick_since_falling: 0,
                    can_coyotee,
                };
            }
            PlayerState::Falling { .. } => {}
        }
    }

    pub fn try_jump(&mut self, input: &input::Axis, velocity: &mut Velocity, jump_speed: f32) {
        if input.pos == 0.0 {
            trace!("no input for jump");
            return;
        }
        match self {
            PlayerState::Grounded {
                speed_at_landing,
                tick_since_landing,
                ..
            } => {
                // Check if the jump should restore the speed.
                if *tick_since_landing <= Self::MAX_TICK_LOSSLESS_JUMP {
                    let Some((current_speed, current_dir)) = velocity.null_y().dist_dir() else {
                        // If we are completly still just jump.
                        trace!("normal jump");
                        velocity.y = jump_speed;
                        self.set_falling(false);
                        return;
                    };

                    // If the speed at landing is less than the current speed don't update it.
                    if current_speed < *speed_at_landing {
                        trace!("Restored jump: current speed: {current_speed}");
                        trace!("\t- current speed: {current_speed}");
                        trace!("\t- restored speed: {speed_at_landing}");
                        let restored_velocity = current_dir * *speed_at_landing;
                        velocity.x = restored_velocity.x;
                        velocity.z = restored_velocity.z;
                    }
                    velocity.y = jump_speed;
                    self.set_falling(false);
                } else {
                    trace!("normal jump");
                    velocity.y = jump_speed;
                    self.set_falling(false);
                }
            }
            PlayerState::Falling {
                tick_since_falling,
                can_coyotee,
            } => {
                // Coyote time.
                if *tick_since_falling <= Self::MAX_TICK_COYOTE_JUMP && *can_coyotee {
                    trace!("coyote jump");
                    velocity.y = jump_speed;
                }
            }
        }
    }
}

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

#[derive(Debug, Clone, Copy, Component, Deref, DerefMut)]
pub struct Velocity(pub Vec3);

#[derive(Component, Deref, DerefMut)]
pub struct Jumped(bool);

#[derive(Component, Deref, DerefMut)]
pub struct Dashed(bool);

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
                Player,
                PbrBundle {
                    mesh: player_mesh,
                    material: player_mat,
                    transform: self
                        .transform
                        .mul_transform(Transform::from_translation(Vec3::Y * 10.0)),
                    ..default()
                },
                // Inputs.
                (
                    input::Mouse::default(),
                    input::MovAxis2::default(),
                    // Vertical for the jump
                    input::AxisSettings(KeyCode::Space, KeyCode::KeyQ),
                    input::Axis::default(),
                ),
                // Physics Related.
                (
                    PlayerState::default(),
                    Velocity(Vec3::ZERO),
                    Collider::capsule_y(0.5, 0.5),
                    CollisionGroups::new(col_layers::PLAYERS, col_layers::ENVIRONEMENT),
                ),
            ))
            .with_children(|cb| {
                cb.spawn((
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
