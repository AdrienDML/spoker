use crate::common::physics::render::{DebugRenderContext, RapierDebugRenderPlugin};
use bevy::ecs::schedule::Stepping;

use crate::prelude::*;

pub struct DebugPlugin;

pub mod player;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RapierDebugRenderPlugin::default())
            .add_systems(Update, draw_gizmos)
            .add_systems(Update, toggle_physics_rendering);

        app.init_resource::<Stepping>()
            .add_systems(Startup, setup_stepping)
            .add_systems(Update, stepping_controlls);
        // add
        app.add_systems(
            Update,
            (
                player::manage_player_control,
                player::update_flycam,
                player::update_noclip,
            ),
        );
    }
}

fn draw_gizmos(mut gizmo: Gizmos) {
    gizmo.arrow(Vec3::ZERO, Vec3::X * 10.0, Color::LIME_GREEN);
    gizmo.arrow(Vec3::ZERO, Vec3::Y * 10.0, Color::RED);
    gizmo.arrow(Vec3::ZERO, Vec3::Z * 10.0, Color::BLUE);
}

fn stepping_controlls(mut stepping: ResMut<Stepping>, keyboard_input: Res<ButtonInput<KeyCode>>) {
    if keyboard_input.just_pressed(KeyCode::Enter) {
        stepping.continue_frame();
    }

    if keyboard_input.just_pressed(KeyCode::F5) {
        if stepping.is_enabled() {
            stepping.disable();
        } else {
            stepping.enable();
        }
    }
}

fn setup_stepping(mut stepping: ResMut<Stepping>) {
    stepping.always_run(Update, stepping_controlls);

    // use crate::systems::player::movement;
    // stepping.enable()
    //     .add_schedule(FixedUpdate)
    //     .set_breakpoint(FixedUpdate, movement::update_player_state)
    //     .set_breakpoint(FixedUpdate, movement::player_jump)
    //     .set_breakpoint(FixedUpdate, movement::player_accelerate)
    //     .set_breakpoint(FixedUpdate, movement::player_move);
}

fn toggle_physics_rendering(
    mut ctx: ResMut<DebugRenderContext>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::F9) {
        ctx.enabled = !ctx.enabled;
    }
}
