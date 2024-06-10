pub mod debug;
pub mod environement;
pub mod player;
pub mod render;
pub mod ui;

use crate::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            environement::EnvironementPlugin,
            player::PlayerPlugin,
            ui::UiPlugin,
        ));
    }
}
