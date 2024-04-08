use crate::settings::{movement::*, player::ControlSettings};
use bevy::prelude::*;

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MovementSettings>()
            .init_resource::<ControlSettings>()
            .init_resource::<MouseSettings>()
            .init_resource::<FlySettings>();
    }
}
