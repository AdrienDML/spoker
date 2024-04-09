use std::f32::consts::PI;

use crate::components::player::*;
use crate::prelude::*;
use crate::settings::movement::*;
use bevy::window::PrimaryWindow;
use bevy_rapier3d::dynamics::ExternalImpulse;
use leafwing_input_manager::prelude::*;

#[cfg(debug_assertions)]
mod debug;
#[cfg(debug_assertions)]
use debug::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            InputManagerPlugin::<Movement>::default(),
            InputManagerPlugin::<Mouse>::default(),
        ))
        .add_systems(Update, collect_inputs)
        .add_systems(FixedUpdate, update);

        #[cfg(debug_assertions)]
        app.add_plugins((
            InputManagerPlugin::<FlyAction>::default(),
            InputManagerPlugin::<PlayerControl>::default(),
        ))
        .add_systems(
            Update,
            (manage_player_control, update_flycam, update_noclip),
        );
    }
}

const SPEED: f32 = 7.0;

pub fn collect_inputs(
    window: Query<&Window, With<PrimaryWindow>>,
    mouse_settings: Res<MouseSettings>,
    mut players: Query<
        (
            &mut Inputs,
            &Transform,
            &ActionState<Movement>,
            &ActionState<Mouse>,
        ),
        With<Player>,
    >,
) {
    let window = window.single();
    for (mut wish, transform, movement, mouse) in &mut players {
        let forward = *transform.forward();
        let right = *transform.right();
        let dir = movement.axis_pair(&Movement::Horizontal).unwrap().xy();
        wish.dir = dir.y * forward + dir.x * right;
        wish.jump = movement.pressed(&Movement::Up);
        wish.crouch = movement.pressed(&Movement::Down);
        wish.mouse += -mouse.axis_pair(&Mouse {}).unwrap().xy()
            / Vec2::new(window.width(), window.height())
            * Vec2::new(mouse_settings.h_sens, mouse_settings.v_sens);
        // Clamp vertical.
        wish.mouse.y = wish.mouse.y.clamp(-PI / 2.0, PI / 2.0);
    }
}

#[no_mangle]
pub fn update(
    time: Res<Time<Virtual>>,
    mut players: RelQuery<
        ParentOf,
        (&Inputs, &mut Transform, &mut ExternalImpulse),
        (Without<NoClip>, Without<FlyCam>, With<Player>),
    >,
    mut player_cam: Query<&mut Transform, (With<PlayerCam>, Without<Player>)>,
) {
    for ((inputs, mut transform, mut impulse), edges) in &mut players {
        edges
            .join::<ParentOf>(&mut player_cam)
            .for_each(|mut cam_transform| {
                impulse.impulse += inputs.dir * time.delta_seconds() * SPEED;
                *cam_transform = cam_transform.with_rotation(Quat::from_rotation_x(inputs.mouse.y));
                *transform = transform.with_rotation(Quat::from_rotation_y(inputs.mouse.x));
            });
    }
}
