use bevy::{prelude::*, window::PrimaryWindow};
use leafwing_input_manager::prelude::*;
use components::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            InputManagerPlugin::<PlayerAction>::default(),
        ))
        .add_systems(Update, update)
        .add_systems(FixedUpdate, fixed_update);
    }
}

const PLAYER_SPEED: f32 = 7f32;

#[no_mangle]
pub fn update(
    window : Query<&Window, With<PrimaryWindow>>,
    time : Res<Time<Virtual>>,
    mouse_settings : Res<MouseSettings>,
    mut players: Query<(Entity, &mut Transform, &ActionState<PlayerAction>), With<Player>>,
    mut player_cam: Query<(&Parent, &mut Transform), (With<PlayerCam>, Without<Player>)>,
) {
    let window = window.single();
    for (player, mut transform, state) in &mut players {
        let cam = {
            let (cam_parent, cam_transform) = player_cam.single_mut();
            if **cam_parent == player {
                Some(cam_transform)
            } else {
                None
            }
        };

        if state.pressed(&PlayerAction::Walk) {
            let forward = transform.forward();
            let left = transform.left();
            let dir = state.axis_pair(&PlayerAction::Walk).unwrap().xy();
            transform.translation -= dir.x * *forward * time.delta_seconds() * PLAYER_SPEED;
            transform.translation -= dir.y * *left * time.delta_seconds() * PLAYER_SPEED;
        }

        if state.pressed(&PlayerAction::Aim) {
            let dir = - state.axis_pair(&PlayerAction::Aim).unwrap().xy()
                / Vec2::new(window.width(), window.height())
                * Vec2::new(mouse_settings.h_sens, mouse_settings.v_sens);

            if let Some(mut cam_tranform) = cam {
                cam_tranform.rotate_z(dir.y);
            }
            transform.rotate_y(dir.x);
        }

        if state.pressed(&PlayerAction::Jump) {

        }
    }
}

#[no_mangle]
pub fn fixed_update() {
    
}
