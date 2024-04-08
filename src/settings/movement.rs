use crate::components::player::{FlyAction, Movement};
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

#[derive(Resource, Clone, Reflect)]
pub struct PlayerCamOffset(Transform);

#[derive(Resource, Clone, Reflect)]
pub struct MovementSettings {
    pub forward: UserInput,
    pub backward: UserInput,
    pub left: UserInput,
    pub right: UserInput,
    pub up: UserInput,
    pub down: UserInput,
}

impl Default for MovementSettings {
    fn default() -> Self {
        Self {
            forward: KeyCode::KeyD.into(),
            backward: KeyCode::KeyS.into(),
            left: KeyCode::KeyA.into(),
            right: KeyCode::KeyH.into(),
            up: KeyCode::Space.into(),
            down: KeyCode::KeyZ.into(),
        }
    }
}

impl From<&MovementSettings> for InputMap<Movement> {
    fn from(val: &MovementSettings) -> Self {
        let UserInput::Single(forward) = val.forward else {
            panic!("Player forward was bound to a non buttonlike input.");
        };
        let UserInput::Single(backward) = val.backward else {
            panic!("Player backward was bound to a non buttonlike input.");
        };
        let UserInput::Single(left) = val.left else {
            panic!("Player left was bound to a non buttonlike input.");
        };
        let UserInput::Single(right) = val.right else {
            panic!("Player right was bound to a non buttonlike input.");
        };
        InputMap::default()
            .insert(
                Movement::Horizontal,
                VirtualDPad {
                    up: forward,
                    down: backward,
                    left,
                    right,
                },
            )
            .insert(Movement::Up, val.up.clone())
            .insert(Movement::Down, val.down.clone())
            .build()
    }
}


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
pub struct FlySettings {
    pub switch_mode: UserInput,
    pub increase_speed: UserInput,
    pub decrease_speed: UserInput,
    pub speed_range: std::ops::Range<f32>,
    pub speed_mult: f32,
    pub speed: f32,
}

impl FlySettings {
    pub fn inc_fly_speed(&self, speed: &mut f32) {
        *speed += *speed * self.speed_mult;
        *speed = speed.clamp(self.speed_range.start, self.speed_range.end);
    }
    pub fn dec_fly_speed(&self, speed: &mut f32) {
        *speed -= *speed * self.speed_mult;
        *speed = speed.clamp(self.speed_range.start, self.speed_range.end);
    }
}

impl Default for FlySettings {
    fn default() -> Self {
        Self {
            switch_mode: MouseButton::Right.into(),
            increase_speed: MouseWheelDirection::Up.into(),
            decrease_speed: MouseWheelDirection::Down.into(),
            speed_range: 1f32..100f32,
            speed_mult: 0.5,
            speed: 10f32,
        }
    }
}

impl From<&FlySettings> for InputMap<FlyAction> {
    fn from(val: &FlySettings) -> Self {
        InputMap::default()
            .insert(FlyAction::SwitchMode, val.switch_mode.clone())
            .insert(FlyAction::IncSpeed, val.increase_speed.clone())
            .insert(FlyAction::DecSpeed, val.decrease_speed.clone())
            .build()
    }
}
