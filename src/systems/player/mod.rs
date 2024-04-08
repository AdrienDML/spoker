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
        .add_systems(Update, update);

        #[cfg(debug_assertions)]
        app.add_plugins((
            InputManagerPlugin::<FlyAction>::default(),
            InputManagerPlugin::<PlayerControl>::default(),
        ))
        .add_systems(
            Update,
            (manage_player_control, update_noclip, update_flycam),
        );
    }
}

const SPEED: f32 = 7.0;

#[no_mangle]
pub fn update(
    window: Query<&Window, With<PrimaryWindow>>,
    time: Res<Time<Virtual>>,
    mouse_settings: Res<MouseSettings>,
    mut players: RelQuery<
        ParentOf,
        (
            &mut Transform,
            &mut ExternalImpulse,
            &ActionState<Movement>,
            &ActionState<Mouse>,
        ),
        (Without<NoClip>, Without<FlyCam>, With<Player>),
    >,
    mut player_cam: Query<&mut Transform, (With<PlayerCam>, Without<Player>)>,
) {
    let window = window.single();
    for ((mut transform, mut impulse, movement, mouse), edges) in &mut players {
        edges.join::<ParentOf>(&mut player_cam).for_each(|mut cam_transform| {
            if movement.pressed(&Movement::Horizontal) {
                let forward = transform.forward();
                let right = transform.right();
                let dir = movement.axis_pair(&Movement::Horizontal).unwrap().xy()
                    * time.delta_seconds()
                    * SPEED;
                impulse.impulse += dir.y * *forward + dir.x * *right;
            }

            if mouse.pressed(&Mouse {}) {
                let dir = -mouse.axis_pair(&Mouse {}).unwrap().xy()
                    / Vec2::new(window.width(), window.height())
                    * Vec2::new(mouse_settings.h_sens, mouse_settings.v_sens);

                cam_transform.rotate_x(dir.y);
                transform.rotate_y(dir.x);
            }

            if movement.pressed(&Movement::Up) {}
        });

    }
}
