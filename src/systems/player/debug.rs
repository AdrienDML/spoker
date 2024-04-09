use std::f32::consts::PI;

use crate::components::player::*;
use crate::prelude::*;
use crate::settings::movement::*;
use bevy_rapier3d::dynamics::RigidBodyDisabled;

pub fn manage_player_control(
    fly_settings: Res<FlySettings>,
    mut commands: Commands,
    players: RelQuery<
        ParentOf,
        (
            Entity,
            &ActionState<PlayerControl>,
            Option<&NoClip>,
            Option<&FlyCam>,
        ),
        With<Player>,
    >,
    mut player_cam: Query<&mut Transform, (With<PlayerCam>, Without<Player>)>,
) {
    for ((player, control, noclip, flycam), childs) in &players {
        childs
            .join::<ParentOf>(&mut player_cam)
            .for_each(|mut cam_transform| {
                if control.just_pressed(&PlayerControl::ToggleNoclip) {
                    if noclip.is_some() {
                        commands
                            .entity(player)
                            .remove::<(NoClip, RigidBodyDisabled)>();
                    } else {
                        commands
                            .entity(player)
                            .insert((NoClip(fly_settings.speed), RigidBodyDisabled));
                    }
                    return;
                }
                if control.just_pressed(&PlayerControl::ToggleFlyCam) {
                    if let Some(fly_cam) = flycam {
                        // Remove the fly cam component.

                        commands.entity(player).remove::<FlyCam>();
                        // Reset the fly cam transform.
                        *cam_transform = fly_cam.return_pos;
                    } else {
                        commands.entity(player).try_insert(FlyCam {
                            speed: fly_settings.speed,
                            return_pos: *cam_transform,
                        });
                    }
                }
            })
    }
}

#[no_mangle]
pub fn update_flycam(
    time: Res<Time<Virtual>>,
    fly_settings: Res<FlySettings>,
    mut players: RelQuery<
        ParentOf,
        (
            &Inputs,
            &ActionState<FlyAction>,
            &mut FlyCam,
        ),
        (With<FlyCam>, With<Player>),
    >,
    mut player_cam: Query<&mut Transform, (With<PlayerCam>, Without<Player>)>,
) {
    for ((inputs, fly_action, mut fly), edges) in &mut players {
        edges
            .join::<ParentOf>(&mut player_cam)
            .for_each(|mut cam_transform| {
                let mut dir = cam_transform.rotation * inputs.dir;
                if !fly_action.pressed(&FlyAction::SwitchMode) {
                    dir = dir.reject_from_normalized(Vec3::Y);
                }
                dir *= time.delta_seconds() * fly.speed;
                cam_transform.translation += dir;

                cam_transform.rotation = Quat::from_rotation_y(inputs.mouse.x)
                * Quat::from_rotation_x(inputs.mouse.y);

                if inputs.jump {
                    cam_transform.translation += Vec3::Y * fly.speed * time.delta_seconds();
                } 
                if inputs.crouch {
                    cam_transform.translation -= Vec3::Y * fly.speed * time.delta_seconds();
                }

                if fly_action.pressed(&FlyAction::IncSpeed) {
                    fly_settings.inc_fly_speed(&mut fly.speed);
                } else if fly_action.pressed(&FlyAction::DecSpeed) {
                    fly_settings.dec_fly_speed(&mut fly.speed);
                }
            })
    }
}

#[no_mangle]
pub fn update_noclip(
    time: Res<Time<Virtual>>,
    fly_settings: Res<FlySettings>,
    mut players: RelQuery<
        ParentOf,
        (
            &Inputs,
            &mut NoClip,
            &mut Transform,
            &ActionState<FlyAction>,
        ),
        (With<NoClip>, With<Player>),
    >,
    mut player_cam: Query<&mut Transform, (With<PlayerCam>, Without<Player>)>,
) {
    for ((inputs, mut noclip, mut transform, fly), edges) in &mut players {
        edges
            .join::<ParentOf>(&mut player_cam)
            .for_each(|mut cam_transform| {
                let dir = inputs.dir
                    * time.delta_seconds()
                    * noclip.0;
                transform.translation += dir;
                *cam_transform = cam_transform.with_rotation(Quat::from_rotation_x(inputs.mouse.y));
                *transform = transform.with_rotation(Quat::from_rotation_y(inputs.mouse.x));

                if inputs.jump {
                    transform.translation += Vec3::Y * noclip.0 * time.delta_seconds();
                } 
                if inputs.crouch {
                    transform.translation -= Vec3::Y * noclip.0 * time.delta_seconds();
                }

                if fly.pressed(&FlyAction::IncSpeed) {
                    fly_settings.inc_fly_speed(&mut noclip.0);
                } else if fly.pressed(&FlyAction::DecSpeed) {
                    fly_settings.dec_fly_speed(&mut noclip.0);
                }
            })
    }
}
