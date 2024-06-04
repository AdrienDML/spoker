use crate::player::components::*;
use crate::prelude::*;
use common::input::{Mouse, MovAxis2, Axis};
use common::physics::prelude::*;

pub type NoDebugFilter = (With<Player>, Without<NoClip>, Without<FlyCam>);
const FLY_SPEED: f32 = 10.0;


pub fn manage_player_control(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    players: Query<(Entity, Option<&NoClip>, Option<&FlyCam>, &Children), With<Player>>,
    mut player_cam: Query<&mut Transform, (With<PlayerCam>, Without<Player>)>,
) {
    for (player, noclip, flycam, childrens) in &players {
        let p_cam = &mut player_cam.iter_many_mut(childrens);
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
    mut players: Query<
        (&Mouse, &MovAxis2, &Axis, &mut FlyCam, &Children),
        (With<FlyCam>, With<Player>),
    >,
    mut player_cam: Query<&mut Transform, (With<PlayerCam>, Without<Player>)>,
) {
    for (mouse, mov, vert, fly, childrens) in &mut players {
        let mut p_cam = player_cam.iter_many_mut(childrens);
        let mut cam_transform = p_cam.fetch_next().expect("No Player Camera Found");
        let scaled_fly_speed = fly.speed * time.delta_seconds();

        // Put direction in camera space.
        let translation = if mouse_button.pressed(MouseButton::Right) {
            mov.movement_3d_in_local(&cam_transform) + vert.value() * Vec3::Y
        } else {
            mouse.yaw() * mov.movement_3d() + vert.value() * Vec3::Y
        };

        if !translation.is_nan() {
            cam_transform.translation += translation * scaled_fly_speed;
        }


        // Set looking direction.
        cam_transform.rotation = mouse.yaw() * mouse.pitch();
    }
}

pub fn update_noclip(
    time: Res<Time<Virtual>>,
    mut players: Query<
        (&Mouse, &MovAxis2, &Axis, &mut NoClip, &mut Transform, &Children),
        (With<NoClip>, Without<FlyCam>, With<Player>),
    >,
    mut player_cam: Query<&mut Transform, (With<PlayerCam>, Without<Player>)>,
) {
    for (mouse, mov, vert, noclip, mut transform, childrens) in &mut players {
        let mut cam = player_cam.iter_many_mut(childrens);
        let mut cam_transform = cam.fetch_next().expect("No Player Camera Found");

        let scaled_speed = noclip.0 * time.delta_seconds();

        let translation = (mov.movement_3d_in_local(&transform) + vert.value() * Vec3::Y) * scaled_speed;
        transform.translation += translation;
        transform.rotation = mouse.yaw();

        // Setting the camera vertical motion.
        *cam_transform = cam_transform.with_rotation(mouse.pitch());
    }
}
