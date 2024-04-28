use crate::prelude::*;
use bevy::utils::HashMap;

mod convert;
pub use convert::Brushable;

pub mod brush_mesh;
use brush_mesh::*;

pub mod operations;
use operations::{BrushMeshOperation, Triangulate};

#[derive(Resource, Default)]
pub struct CsgContext {
    cached_meshes: HashMap<Entity, Handle<Mesh>>,
}

pub struct CsgPlugin;

impl Plugin for CsgPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CsgContext>()
            .add_systems(Update, on_brush_added);
    }
}

fn on_brush_added(
    mut commands: Commands,
    mut context: ResMut<CsgContext>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<(Entity, &BrushMesh), Added<BrushMesh>>,
) {
    for (entity, added_brush) in &query {
        let mesh = meshes.add(Triangulate::EarClip.apply(added_brush));
        context.cached_meshes.insert(entity, mesh.clone());
        commands.entity(entity).insert(PbrBundle {
            mesh,
            material: materials.add(StandardMaterial {
                base_color: Color::GRAY,
                ..default()
            }),
            ..default()
        });
    }
}
