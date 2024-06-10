use super::{BrushMesh, CsgLeaf, CsgNode, CsgOp};
use bevy::ecs::system::EntityCommands;
use crate::prelude::*;

pub trait CsgCommandsExt {
    fn spawn_csg_node(&mut self, node: SpawnCsgNode) -> EntityCommands<'_>;
}

impl CsgCommandsExt for Commands<'_, '_> {
    fn spawn_csg_node(&mut self, node: SpawnCsgNode) -> EntityCommands<'_> {
        let mut id = Entity::PLACEHOLDER;
        self.entity(node.parent).with_children(|parent| {
            let mut spawned =
                parent.spawn((TransformBundle::from_transform(node.tranform), node.op));
            if let Some(brush) = node.brush {
                spawned.insert((CsgLeaf, brush));
            } else {
                spawned.insert(CsgNode);
            }
            id = spawned.id();
        });
        return self.entity(id);
    }
}

pub struct SpawnCsgNode {
    parent: Entity,
    tranform: Transform,
    op: CsgOp,
    brush: Option<Handle<BrushMesh>>,
}
