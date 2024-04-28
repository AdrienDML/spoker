use crate::components::player::*;
use crate::prelude::*;
use bevy_rapier3d::dynamics::RigidBodyDisabled;
use common::input::{Mouse, MovAxis3};

const FLY_SPEED: f32 = 10.0;

pub fn manage_player_control(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    players: Query<(Entity, Option<&NoClip>, Option<&FlyCam>, &Children), With<Player>>,
    mut player_cam: Query<&mut Transform, (With<PlayerCam>, Without<Player>)>,
) {
    for (player, noclip, flycam, childrens) in &players {
        let mut p_cam = player_cam.iter_many_mut(childrens);
        let mut cam_transform = p_cam.fetch_next().expect("No Player Camera Found");
        if keys.just_pressed(KeyCode::F1) {
            if noclip.is_some() {
                commands
                    .entity(player)
                    .remove::<(NoClip, RigidBodyDisabled)>();
            } else {
                commands
                    .entity(player)
                    .insert((NoClip(FLY_SPEED), RigidBodyDisabled));
            }
            return;
        }
        if keys.just_pressed(KeyCode::F2) {
            if let Some(fly_cam) = flycam {
                // Remove the fly cam component.

                commands.entity(player).remove::<FlyCam>();
                // Reset the fly cam transform.
                *cam_transform = fly_cam.return_pos;
            } else {
                commands.entity(player).try_insert(FlyCam {
                    speed: FLY_SPEED,
                    return_pos: *cam_transform,
                });
            }
        }
    }
}

pub fn update_flycam(
    time: Res<Time<Virtual>>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut players: Query<(&Mouse, &MovAxis3, &mut FlyCam, &Children), (With<FlyCam>, With<Player>)>,
    mut player_cam: Query<&mut Transform, (With<PlayerCam>, Without<Player>)>,
) {
    for (mouse, mov, fly, childrens) in &mut players {
        let mut p_cam = player_cam.iter_many_mut(childrens);
        let mut cam_transform = p_cam.fetch_next().expect("No Player Camera Found");

        let scaled_fly_speed = fly.speed * time.delta_seconds();
        // Put direction in camera space.
        if mouse_button.pressed(MouseButton::Right) {
            mov.apply_dir_in_local(&mut cam_transform, scaled_fly_speed)
        } else {
            mov.apply_dir(&mut cam_transform, scaled_fly_speed)
        }

        // Set looking direction.
        mouse.apply_mouse_rot(&mut cam_transform);
    }
}

pub fn update_noclip(
    time: Res<Time<Virtual>>,
    mut players: Query<
        (&Mouse, &MovAxis3, &mut NoClip, &mut Transform, &Children),
        (With<NoClip>, With<Player>),
    >,
    mut player_cam: Query<&mut Transform, (With<PlayerCam>, Without<Player>)>,
) {
    for (mouse, mov, noclip, mut transform, childrens) in &mut players {
        let mut cam = player_cam.iter_many_mut(childrens);
        let mut cam_transform = cam.fetch_next().expect("No Player Camera Found");

        let scaled_speed = noclip.0 * time.delta_seconds();

        mov.apply_dir(&mut transform, scaled_speed);

        *cam_transform = cam_transform.with_rotation(mouse.pitch());
        *transform = transform.with_rotation(mouse.yaw());
    }
}
