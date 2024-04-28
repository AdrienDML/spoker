use crate::prelude::*;

pub struct DebugPlugin; 

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(RapierDebugRenderPlugin::default())
            .add_systems(Update, draw_gizmos);
    }
}


fn draw_gizmos(
    mut gizmo: Gizmos,
) {
    gizmo.arrow(Vec3::ZERO, Vec3::X * 10.0, Color::LIME_GREEN);
    gizmo.arrow(Vec3::ZERO, Vec3::Y * 10.0, Color::RED);
    gizmo.arrow(Vec3::ZERO, Vec3::Z * 10.0, Color::BLUE);
}
