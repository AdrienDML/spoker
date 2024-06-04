mod ui;
pub use ui::*;
mod input;
pub use input::*;
mod vec3;
pub use vec3::*;
mod commands;
pub use commands::*;
mod f32;
pub use f32::*;
mod prev;
pub use prev::*;
mod setting;
pub use setting::*;

#[macro_export]
macro_rules! val {
    ($v:tt px) => {
        Val::Px($v)
    };

    ($v:tt %) => {
        Val::Percent($v)
    }
}
