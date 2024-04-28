use crate::components::player::*;
use crate::prelude::*;
use bevy_rapier3d::dynamics::ExternalImpulse;
use common::input::{Mouse, MovAxis3};

#[cfg(debug_assertions)]
mod debug;
#[cfg(debug_assertions)]
use debug::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, update);

        // Add debug cameras.
        #[cfg(debug_assertions)]
        app.add_systems(
            Update,
            (manage_player_control, update_flycam, update_noclip),
        );
    }
}

const SPEED: f32 = 7.0;

pub fn update(
    time: Res<Time<Virtual>>,
    mut players: Query<
        (
            &Mouse,
            &MovAxis3,
            &mut Transform,
            &mut ExternalImpulse,
            &Children,
        ),
        (Without<NoClip>, Without<FlyCam>, With<Player>),
    >,
    mut player_cam: Query<&mut Transform, (With<PlayerCam>, Without<Player>)>,
) {
    for (mouse, axis, mut transform, mut impulse, childrens) in &mut players {
        let mut p_cam = player_cam.iter_many_mut(childrens);
        let mut cam_transform = p_cam.fetch_next().expect("No Player Camera Found");

        let scaled_speed = SPEED * time.delta_seconds();

        impulse.impulse += axis.total_movement_in_local(&transform) * scaled_speed;

        *cam_transform = cam_transform.with_rotation(mouse.pitch());
        *transform = transform.with_rotation(mouse.yaw());
    }
}
