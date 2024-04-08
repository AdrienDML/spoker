use crate::components::player::*;
use crate::prelude::*;
use crate::settings::movement::*;
use bevy::window::PrimaryWindow;
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
    window: Query<&Window, With<PrimaryWindow>>,
    time: Res<Time<Virtual>>,
    mouse_settings: Res<MouseSettings>,
    fly_settings: Res<FlySettings>,
    mut players: RelQuery<
        ParentOf,
        (
            &Transform,
            &ActionState<Movement>,
            &ActionState<Mouse>,
            &ActionState<FlyAction>,
            &mut FlyCam,
        ),
        (With<FlyCam>, With<Player>),
    >,
    mut player_cam: Query<&mut Transform, (With<PlayerCam>, Without<Player>)>,
) {
    let window = window.single();
    for ((transform, movement, mouse, fly_action, mut fly), edges) in &mut players {
        edges
            .join::<ParentOf>(&mut player_cam)
            .for_each(|mut cam_transform| {
                if movement.pressed(&Movement::Horizontal) {
                    let mut forward = *cam_transform.forward();
                    if !fly_action.pressed(&FlyAction::SwitchMode) {
                        forward = forward.reject_from_normalized(*transform.up());
                    }
                    let left = *cam_transform.local_x();
                    let dir = movement.axis_pair(&Movement::Horizontal).unwrap().xy()
                        * time.delta_seconds()
                        * fly.speed;
                    cam_transform.translation += dir.y * forward + dir.x * left;
                }

                if mouse.pressed(&Mouse {}) {
                    let dir = -mouse.axis_pair(&Mouse {}).unwrap().xy()
                        / Vec2::new(window.width(), window.height())
                        * Vec2::new(mouse_settings.h_sens, mouse_settings.v_sens);

                    cam_transform.rotate_local_x(dir.y);
                    cam_transform.rotate_y(dir.x);
                }

                if movement.pressed(&Movement::Up) {
                    cam_transform.translation += Vec3::Y * fly.speed * time.delta_seconds();
                } else if movement.pressed(&Movement::Down) {
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
    window: Query<&Window, With<PrimaryWindow>>,
    time: Res<Time<Virtual>>,
    mouse_settings: Res<MouseSettings>,
    fly_settings: Res<FlySettings>,
    mut players: RelQuery<
        ParentOf,
        (
            &mut NoClip,
            &mut Transform,
            &ActionState<Movement>,
            &ActionState<Mouse>,
            &ActionState<FlyAction>,
        ),
        (With<NoClip>, With<Player>),
    >,
    mut player_cam: Query<&mut Transform, (With<PlayerCam>, Without<Player>)>,
) {
    let window = window.single();
    for ((mut noclip, mut transform, movement, mouse, fly), edges) in &mut players {
        edges
            .join::<ParentOf>(&mut player_cam)
            .for_each(|mut cam_transform| {
                if movement.pressed(&Movement::Horizontal) {
                    let mut forward = *cam_transform.forward();
                    if !fly.pressed(&FlyAction::SwitchMode) {
                        forward = forward.reject_from_normalized(*transform.up());
                    }
                    let right = *transform.right();
                    let dir = movement.axis_pair(&Movement::Horizontal).unwrap().xy()
                        * time.delta_seconds()
                        * noclip.0;
                    transform.translation += dir.y * forward + dir.x * right;
                }

                if mouse.pressed(&Mouse {}) {
                    let dir = -mouse.axis_pair(&Mouse {}).unwrap().xy()
                        / Vec2::new(window.width(), window.height())
                        * Vec2::new(mouse_settings.h_sens, mouse_settings.v_sens);

                    cam_transform.rotate_x(dir.y);
                    transform.rotate_y(dir.x);
                }

                if movement.pressed(&Movement::Up) {
                    transform.translation += Vec3::Y * noclip.0 * time.delta_seconds();
                } else if movement.pressed(&Movement::Down) {
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
