use aery::relation::RelationId;

use crate::prelude::*;

pub struct HierachyRelationsPlugin;

impl Plugin for HierachyRelationsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, update_hierachy_rels);
    }
}

#[derive(Relation)]
pub struct ChildOf;

#[derive(Relation)]
#[aery(Poly)]
pub struct ParentOf;

/// R: Relations, D: QueryData, F: QueryFilter.
pub type RelQuery<'w, 's, R, D, F> = Query<'w, 's, (D, Relations<R>), F>;

pub trait ChildBuilderExt {
    fn spawn_with_rel<B: Bundle>(&mut self, bundle: B) -> EntityWorldMut<'_>;
}
impl<'w> ChildBuilderExt for WorldChildBuilder<'w> {
    fn spawn_with_rel<B: Bundle>(&mut self, bundle: B) -> EntityWorldMut<'_> {
        let id = self.parent_entity();
        let mut child = self.spawn(bundle);
        child.set::<ParentOf>(id);
        child
    }
}

fn update_hierachy_rels(
    mut commands: Commands,
    mut events: EventReader<TargetEvent>,
) {
    let child_id: RelationId = RelationId::of::<ChildOf>();
    let parent_id: RelationId = RelationId::of::<ParentOf>();
    for event in events.read() {
        match event {
            TargetEvent {
                host,
                target,
                target_op: Op::Set,
                relation_id,
            } if *relation_id == child_id => {
                commands.get_entity(*target).unwrap().set::<ParentOf>(*host);
            }
            TargetEvent {
                host,
                target,
                target_op: Op::Unset,
                relation_id,
            } if *relation_id == child_id  => {
                commands.get_entity(*target).unwrap().unset::<ParentOf>(*host);
            }
            TargetEvent {
                host,
                target,
                target_op: Op::Set,
                relation_id,
            } if *relation_id == parent_id => {
                commands.get_entity(*target).unwrap().set::<ChildOf>(*host);
            }
            TargetEvent {
                host,
                target,
                target_op: Op::Unset,
                relation_id,
            }  if *relation_id == parent_id => {
                commands.get_entity(*target).unwrap().unset::<ChildOf>(*host);
            }
            _ => {}

        }
    }
}
