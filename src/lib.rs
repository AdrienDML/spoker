pub mod common;
pub mod editor;
pub mod game;
pub mod main_menu;

pub mod prelude {
    pub use super::{
        val,
        common::{
            ext::*,
            ui::{
                ui_builder::{UiBuilder, UiBuilderExt, UiContextRoot, UiRoot},
                ui_commands::*,
                ui_style::*,
                widgets::{foldable::UiFoldableExt, prelude::*},
                SickleUiPlugin,
            },
        },
        AppState,
    };
    pub use bevy_infinite_grid::*;
    pub use bevy::prelude::*;
    pub use std::f32::consts::*;
}

use bevy::window::{Cursor, CursorGrabMode};
use prelude::*;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    #[default]
    MainMenu,
    Editor,
    InGame,
}

pub fn run(start_state: Option<AppState>) {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Spoker".to_string(),
                cursor: Cursor {
                    grab_mode: CursorGrabMode::Locked,
                    visible: false,
                    ..default()
                },
                ..default()
            }),
            ..default()
        }),
        common::input::InputPlugin,
        common::physics::PhysicsPlugin::default(),
        bevy_infinite_grid::InfiniteGridPlugin,
        game::GamePlugin,
        editor::EditorPlugin,
        main_menu::MainMenuPlugin,
    ))
    .insert_state(start_state.unwrap_or_default())
    .insert_resource(Msaa::Sample8);
}
