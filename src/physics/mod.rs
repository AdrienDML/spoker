use bevy::prelude::*;

pub mod bsp;
pub mod physmat;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, bsp::setup_map);
    }
}
