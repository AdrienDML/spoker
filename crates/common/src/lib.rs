pub mod input;

mod ext;
pub mod debug;
pub mod physics;

pub mod ui {
    pub use sickle_ui::*;
}

pub mod prelude {
    pub use std::f32::consts::*;
    pub use bevy::prelude::*;
    pub use super::ext::*;
    pub use super::ui::{
        widgets::{prelude::*, foldable::UiFoldableExt},
        ui_builder::{
            UiBuilder,
            UiBuilderExt,
            UiRoot,
            UiContextRoot,
        },
        ui_commands::*,
        ui_style::*,
        SickleUiPlugin,
    };
}
