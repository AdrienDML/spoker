use crate::{debug::player::NoDebugFilter, player::components::*, prelude::*};

use common::{
    input::{Axis, Mouse, MovAxis2},
    physics::{
        cast_shape, col_layers,
        geometry::{Collider, CollisionGroups},
        move_and_collide,
        plugin::RapierContext,
        CollisionSettings,
    },
};

#[derive(Debug, SystemSet, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PlayerPhysicsSystem;

const AIR_ACCELERATION: f32 = 0.5;

const COLLISION_SETTINGS: CollisionSettings = CollisionSettings {
    skin_width: 0.1,
    collision_group: CollisionGroups::new(col_layers::PLAYERS, col_layers::ENVIRONEMENT),
};

pub fn track_player_settings() {}

pub fn update_view(
    mut players: Query<(&Mouse, &mut Transform, &Children), NoDebugFilter>,
    mut player_cam: Query<&mut Transform, (With<PlayerCam>, Without<Player>)>,
) {
    for (mouse, mut transform, childrens) in &mut players {
        let mut p_cam = player_cam.iter_many_mut(childrens);
        let Some(mut cam_transform) = p_cam.fetch_next() else {
            warn!("No Player Camera Found");
            return;
        };

        *cam_transform = cam_transform.with_rotation(mouse.pitch());
        *transform = transform.with_rotation(mouse.yaw());
    }
}

pub fn update_player_state(
    physics_context: Res<RapierContext>,
    ground_prop: Res<PlayerGroundProperties>,
    mut players: Query<(Entity, &Collider, &Transform, &Velocity, &mut PlayerState, Option<&PlayerGroundProperties>), NoDebugFilter>,
) {
    trace!("\n===== Update State =====\n");
    for (player, collider, transform, velocity, mut state, _ground_prop) in &mut players {
        let ground_prop = _ground_prop.unwrap_or(&ground_prop);
        let Some(hit) = cast_shape(
            physics_context.as_ref(),
            COLLISION_SETTINGS,
            player,
            collider,
            transform,
            -Vec3::Y,
            1.0,
        ) else {
            trace!("CastShape Missed.");
            // No ground hit set to falling.
            state.set_falling(true);
            trace!("{state:?}");
            continue;
        };
        trace!("CastShape hit:");
        trace!("{hit:?}");

        if hit.toi < 0.1 && hit.normal.angle_between(Vec3::Y) < ground_prop.max_slope_traction_angle {
            state.set_grounded(velocity, hit.normal);
        }
        trace!("{state:?}");
    }
}

// Player jump.
pub fn player_jump(
    jump_prop: Res<PlayerJumpProperties>,
    mut players: Query<(&mut PlayerState, &Axis, &mut Velocity, Option<&PlayerJumpProperties>), NoDebugFilter>) {
    trace!("\n===== Update Jump ===== \n");
    for (mut state, input, mut velocity, _jump_prop) in &mut players {
        let jump_prop = _jump_prop.unwrap_or(&jump_prop);
        state.try_jump(input, velocity.as_mut(), jump_prop.jump_speed);
    }
}

// Player acceleration code.
pub fn player_accelerate(
    ground_prop: Res<PlayerGroundProperties>,
    jump_prop: Res<PlayerJumpProperties>,
    time: Res<Time<Fixed>>,
    mut players: Query<
        (
            &MovAxis2,
            &Transform,
            &mut Velocity,
            &PlayerState,
            Option<&PlayerGroundProperties>,
            Option<&PlayerJumpProperties>,
        ),
        NoDebugFilter,
    >,
) {
    trace!("\n===== Update Acceleration ===== \n");

    let dt = time.delta_seconds();
    for (input, transform, mut velocity, state, _ground_prop, _jump_prop) in &mut players {
        let ground_prop = _ground_prop.unwrap_or(&ground_prop);
        let jump_prop = _jump_prop.unwrap_or(&jump_prop);
        let wish_vel = input.movement_3d_in_local(transform);
        trace!("Before Acceleration:{velocity:?}");
        match *state {
            PlayerState::Grounded {
                tick_since_landing, ..
            } => {
                // Ground Acceleration.
                if let Some((mut current_speed, current_direction)) = velocity.dist_dir() {
                    // Only apply friction if we are on the ground for more than one tick.
                    if tick_since_landing > 0 {
                        current_speed *= f32::exp(-ground_prop.friction_coef * dt);
                        if current_speed < ground_prop.min_speed {
                            current_speed = 0.0;
                        }
                        **velocity = current_speed * current_direction;
                    }
                }
                if let Some((_, wish_dir)) = wish_vel.dist_dir() {
                    let mut speed_on_wish_dir = wish_dir.dot(**velocity);
                    speed_on_wish_dir = ground_prop.max_speed 
                        + (speed_on_wish_dir - ground_prop.max_speed)
                            * f32::exp(-ground_prop.acceleration_coef * dt);
                    **velocity = wish_dir * speed_on_wish_dir;
                }
            }
            PlayerState::Falling { .. } => {
                **velocity += wish_vel * AIR_ACCELERATION * dt;
                velocity.y -= jump_prop.gravity * dt;
            }
        }
        trace!("After Acceleration: {velocity:?}");
    }
}

pub fn player_move(
    time: Res<Time<Fixed>>,
    physics_context: Res<RapierContext>,
    mut players: Query<(Entity, &mut Transform, &mut Velocity, &Collider), NoDebugFilter>,
) {
    let dt = time.delta_seconds();
    trace!("\n===== Update Move ===== \n");
    for (player, mut transform, mut velocity, collider) in &mut players {
        let Some((mut current_speed, mut current_dir)) = velocity.dist_dir() else {
            return;
        };
        current_speed *= dt;

        let mut depth = 0;
        trace!("Initial state:");
        trace!("\t dir: {current_speed}");
        trace!("\t speed: {current_speed}");
        while let Some(hit) = move_and_collide(
            physics_context.as_ref(),
            COLLISION_SETTINGS,
            player,
            collider,
            transform.as_mut(),
            current_dir,
            current_speed,
        ) {
            trace!("Iteration: {}", depth);
            if depth > 5 {
                trace!("Iteration depth limit reached.");
                **velocity = Vec3::ZERO;
                return;
            }

            **velocity = velocity.reject_from(hit.normal);
            current_speed -= hit.toi;
            current_dir = current_dir.reject_from(hit.normal);
            trace!("State:");
            trace!("\t dir: {current_speed}");
            trace!("\t speed: {current_speed}");
            depth += 1;
        }

        // Add remaining speed if there is any
        if current_speed > f32::EPSILON {
            trace!("No more hits add remaining velocity.");
            transform.translation += current_dir * current_speed;
            trace!("Position: {}", transform.translation);
        }
    }
}
