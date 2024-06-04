use crate::prelude::*;

use common::physics::plugin::PhysicsSet;

pub mod components;
pub mod systems;
use systems::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            FixedUpdate,
            systems::movement::PlayerPhysicsSystem.after(PhysicsSet::SyncBackend),
        )
        .add_plugins((
                SettingPlugin::<components::PlayerJumpSettings>::default(),
                SettingPlugin::<components::PlayerGroundSettings>::default(),
            ))
        .add_systems(Update, movement::update_view)
        .add_systems(
            FixedUpdate,
            (
                movement::update_player_state,
                movement::player_jump,
                movement::player_accelerate,
                movement::player_move,
            )
                .chain(),
        );
    }
}
