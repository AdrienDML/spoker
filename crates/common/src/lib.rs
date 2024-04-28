pub mod input;

mod ext;
pub mod prelude {
    pub use bevy::prelude::*;
    pub use super::ext::*;
}
