#![allow(ambiguous_glob_reexports)]
#![allow(clippy::type_complexity)]

pub mod components;
pub mod settings;
pub mod systems;
pub mod ext;

pub mod prelude {
    pub use super::ext::*;
    pub use bevy::prelude::*;
    pub use aery::prelude::*;
    pub use bevy_rapier3d::prelude::*;
    pub use leafwing_input_manager::prelude::*;
}
