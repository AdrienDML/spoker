use common::prelude::*;

use bevy::window::{CursorGrabMode, PrimaryWindow};

mod speed_text;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, speed_text::setup)
            .add_systems(Update, release_cursor_on_esc)
            .add_systems(PostUpdate, speed_text::track_speed);
    }
}

pub fn release_cursor_on_esc(
    esc_input: Res<ButtonInput<KeyCode>>,
    mut window: Query<&mut Window, With<PrimaryWindow>>,
) {
    let mut window = window.single_mut();
    if esc_input.just_pressed(KeyCode::Escape) {
        let cursor = &mut window.cursor;
        match cursor.grab_mode {
            CursorGrabMode::Locked => {
                cursor.grab_mode = CursorGrabMode::None;
                cursor.visible = true;
            }
            _ => {
                cursor.grab_mode = CursorGrabMode::Locked;
                cursor.visible = false;
            }
        }
    }
}
