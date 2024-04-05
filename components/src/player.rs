use bevy::{ecs::system::Command, prelude::*};
use leafwing_input_manager::prelude::*;
use bitflags::bitflags;
use crate::settings::*;

#[derive(Component)]
pub struct Player;

#[derive(Reflect, Actionlike, Clone, Eq, PartialEq, Hash)]
pub enum PlayerAction {
    Walk,
    Scope,
    Jump,
    Crouch,
    Dash,
    Aim,
}

#[derive(Component, Default, Clone)]
pub struct PlayerState(u32);
bitflags! {
    impl PlayerState: u32 {
        // If the player is on the ground this tick.
        const GROUNDED = 0b001;

        // If the player is on the ground this tick.
        const AIR_BORN = 0b010;

        // If the player is on a wall this tick.
        const ON_WALL = 0b011;

        // Detect if the player can catch an corner for a corner boost.
        const CAN_CATCH_EDGE = 0b100;
    }
}

// Update the player velocity variable and set jump and dash flags.
#[derive(Component)]
pub struct PlayerCam;

#[derive(Component, Default, Deref, DerefMut)]
pub struct Velocity(Vec3);

pub struct SpawnPlayerCmd {
    pub transform: Transform,
    pub cam_offset: Vec3,
}

impl Command for SpawnPlayerCmd {
    fn apply(self, world: &mut World) {
        let Some(input_settings) = world.get_resource::<MovementSettings>() else {
            error!("Tried to spawn player but input settings were not present.");
            return;
        };
        world
            .spawn((
                TransformBundle {
                    local: self.transform,
                    ..default()
                },
                Player,
                PlayerState::default(),
                InputManagerBundle::<PlayerAction> {
                    input_map: input_settings.clone().into(),
                    ..default()
                },
            ))
            .with_children(|cb| {
                let mut cam_transform = Transform::from_translation(self.cam_offset);
                cam_transform.look_to(Vec3::X, Vec3::Y);

                cb.spawn((
                    Camera3dBundle {
                        transform: cam_transform,
                        ..default()
                    },
                    PlayerCam,
                ));
            });
    }
}
