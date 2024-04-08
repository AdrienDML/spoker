use crate::components::player::PlayerControl;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub struct PlayerAbilities {
    pub dash: UserInput,
    pub blink: UserInput,
}

#[derive(Resource, Clone)]
pub struct ControlSettings {
    toggle_flycam: UserInput,
    toggle_noclip: UserInput,
}

impl Default for ControlSettings {
    fn default() -> ControlSettings {
        ControlSettings {
            toggle_flycam: KeyCode::F1.into(),
            toggle_noclip: KeyCode::F2.into(),
        }
    }
}

impl From<&ControlSettings> for InputMap<PlayerControl> {
    fn from(val: &ControlSettings) -> Self {
        InputMap::default()
            .insert(PlayerControl::ToggleFlyCam, val.toggle_flycam.clone())
            .insert(PlayerControl::ToggleNoclip, val.toggle_noclip.clone())
            .build()
    }
}
