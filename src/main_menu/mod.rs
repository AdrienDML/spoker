use crate::prelude::*;

pub enum MainMenuState {
    Landing,
    Options,
}

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
    }
}

