use bevy::ecs::schedule::SystemSet;

pub mod csg;
pub mod camera;
pub mod select;
pub mod r#move;
pub mod ui;

#[derive(SystemSet, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct RunOnMapFocused;

pub mod prelude {
    pub use bevy::prelude::*;
    pub use common::prelude::*;
}
