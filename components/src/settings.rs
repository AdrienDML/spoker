use bevy::prelude::*;
use crate::player::PlayerAction;
use leafwing_input_manager::prelude::*;

#[derive(Resource, Clone, Reflect)]
pub struct MouseSettings {
    pub h_sens: f32,
    pub v_sens: f32,
}

impl Default for MouseSettings {
    fn default() -> Self {
        Self {
            h_sens: 1f32,
            v_sens: 1f32,
        }
    }
}

#[derive(Resource, Clone)]
pub struct MovementSettings {
    pub forward: UserInput,
    pub backward: UserInput,
    pub left: UserInput,
    pub right: UserInput,
    pub jump: UserInput,
    pub dash: UserInput,
    pub crouch: UserInput,
}

impl Default for MovementSettings {
    fn default() -> Self {
        Self {
            forward: KeyCode::KeyD.into(),
            backward: KeyCode::KeyS.into(),
            left: KeyCode::KeyA.into(),
            right: KeyCode::KeyH.into(),
            jump: KeyCode::Space.into(),
            dash: MouseButton::Right.into(),
            crouch: KeyCode::KeyZ.into(),
        }
    }
}

#[allow(clippy::from_over_into)]
impl Into<InputMap<PlayerAction>> for MovementSettings {
    fn into(self) -> InputMap<PlayerAction> {
        let UserInput::Single(forward) = self.forward else {
            panic!("Player forward was bound to a non buttonlike input.");
        };
        let UserInput::Single(backward) = self.backward else {
            panic!("Player backward was bound to a non buttonlike input.");
        };
        let UserInput::Single(left) = self.left else {
            panic!("Player left was bound to a non buttonlike input.");
        };
        let UserInput::Single(right) = self.right else {
            panic!("Player right was bound to a non buttonlike input.");
        };
        InputMap::default()
            .insert(
                PlayerAction::Walk,
                VirtualDPad {
                    up: forward,
                    down: backward,
                    left,
                    right,
                },
            )
            .insert(PlayerAction::Jump, self.jump)
            .insert(PlayerAction::Dash, self.dash)
            .insert(PlayerAction::Crouch, self.crouch)
            .insert(PlayerAction::Aim, DualAxis::mouse_motion())
            .build()
    }
}
