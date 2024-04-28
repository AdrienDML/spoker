#![allow(ambiguous_glob_reexports)]
#![allow(clippy::type_complexity)]

pub mod components;
pub mod systems;
pub mod ext;
pub mod render;

pub mod prelude {
    pub use super::ext::*;
    pub use common::prelude::*;
    pub use bevy::prelude::*;
    pub use aery::prelude::*;
    pub use bevy_rapier3d::prelude::*;
}
