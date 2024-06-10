use crate::common::physics::plugin::PhysicsSet;
use crate::{prelude::*, AppState};

pub mod components;
pub mod systems;
use systems::*;

use self::movement::PlayerPhysicsSystem;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            FixedUpdate,
            PlayerPhysicsSystem
                .after(PhysicsSet::SyncBackend)
                .run_if(in_state(AppState::InGame)),
        )
        .add_plugins((
            SettingPlugin::<components::PlayerJumpSettings>::default(),
            SettingPlugin::<components::PlayerGroundSettings>::default(),
        ))
        .add_systems(Update, movement::update_view.in_set(PlayerPhysicsSystem))
        .add_systems(
            FixedUpdate,
            (
                movement::update_player_state,
                movement::player_jump,
                movement::player_accelerate,
                movement::player_move,
            )
                .chain()
                .in_set(PlayerPhysicsSystem),
        );
    }
}
