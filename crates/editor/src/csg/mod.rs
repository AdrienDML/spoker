use crate::prelude::*;

pub mod convert;
pub mod hierarchy;
pub use convert::Brushable;

pub mod brush_mesh;
use brush_mesh::*;

pub mod operations;
pub struct CsgPlugin;

impl Plugin for CsgPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<BrushMesh>()
            .add_systems(Startup, setup_csg_root)
            .add_systems(Update, on_brush_added);
    }
}

#[derive(Component)]
pub struct CsgRoot;

#[derive(Component)]
pub struct CsgNode;

#[derive(Component)]
pub struct CsgLeaf;

#[derive(Component)]
#[component(storage = "SparseSet")]
struct Dirty;

// Csg operation is add by default set this component on a brush to put it in substract mode.
#[derive(Component, Default, Clone, Copy)]
pub enum CsgOp {
    #[default]
    Add,
    Substract,
}

fn setup_csg_root(mut commands: Commands, mut brushes: ResMut<Assets<BrushMesh>>) {
    commands.spawn((
        TransformBundle::default(),
        CsgRoot,
        brushes.add(BrushMesh::empty()),
    ));
}

fn on_brush_added(
    mut commands: Commands,
    brushes: Res<Assets<BrushMesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    leaf_query: Query<(Entity, Option<&Name>, &CsgOp), (Added<Handle<BrushMesh>>, With<CsgLeaf>)>,
) {
    for (entity, name, op) in &leaf_query {
        commands.entity(entity).insert(Dirty);

        let mut brush = commands.entity(entity);
        // Set the brush name to "Unnamed".
        if name.is_none() {
            brush.insert(Name::new("Unnamed"));
        };
    }
}

fn propagate_dirty(mut commands: Commands, dirtied: Entity, dirty_query: Query<(Entity, &Parent), Or<(With<CsgRoot>, With<CsgNode>)>>, dirty_leaf: Query<&Parent, With<CsgLeaf>>) {
    let Ok(mut dirty_parent) = dirty_leaf.get(dirtied) else {
        error!("Dirty entity not found {dirtied:?}.");
        return;
    };
    while let Ok((to_dirty, parent)) = dirty_query.get(**dirty_parent) {
        commands.entity(dirtied).insert(Dirty);
        dirty_parent = parent;
    }
}

pub fn undirty_tree(
    mut commands: Commands,
    mut brushes: ResMut<Assets<BrushMesh>>,
    mut dirty_root: Query<(&Children, &Handle<BrushMesh>), (With<CsgRoot>, With<Dirty>)>,
    nodes: Query<(&Transform, Option<&Children>, &Handle<BrushMesh>, Option<&Dirty>, &CsgOp), Or<(With<CsgNode>, With<CsgLeaf>)>>,
) {
    let Ok((top_childrens, brush)) = dirty_root.get_single() else {
        return;
    };
    for child in top_childrens {
        undirty_node(*child, commands, brushes,  nodes);
    }
    nodes.iter_many(top_childrens);
}

fn undirty_node<'a>(
    node: Entity, 
    mut commands: Commands,
    mut burshes: ResMut<Assets<BrushMesh>>,
    nodes: Query<(&Transform, Option<&Children>, &Handle<BrushMesh>, Option<&Dirty>, &CsgOp), Or<(With<CsgNode>, With<CsgLeaf>)>>,
) -> Option<(Transform, &'a Handle<BrushMesh>, CsgOp)> {
    let brushes: Vec<(&Handle<BrushMesh>, CsgOp)> = Vec::new();
    let (transform, childrens, brush, dirty, op) = nodes.get(node).ok()?; 

    // If the node is not dirty or it is a leaf there is nothing to do. 
    if dirty.is_none() || childrens.is_none() {
        return Some((*transform, brush, *op));
    }

    

    return Some((*transform, brush, *op));
}


