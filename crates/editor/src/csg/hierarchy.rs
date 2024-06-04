use bevy::{hierarchy::HierarchyEvent, utils::HashSet};
use crate::prelude::*;

use super::{CsgNode, CsgRoot};

#[derive(Event)]
pub enum CsgHierarchyEvent {
    ChildAdded {
        child: Entity,
        parent: Entity,
    },
    ChildRemoved {
        child: Entity,
        old_parent: Entity,
    },
    ChildMoved {
        child: Entity,
        old_parent: Entity,
        new_parent: Entity,
    },
}


pub fn filter_csg_hierarchy_events(
    mut h_ui_event: EventWriter<CsgHierarchyEvent>,
    mut h_events: EventReader<HierarchyEvent>,
    csg_internal_node: Query<Entity, Or<(With<CsgRoot>, With<CsgNode>)>>,
) {
    let csg_nodes: HashSet<Entity> = csg_internal_node.iter().collect();
    for hierarchy_event in h_events.read() {
        match hierarchy_event {
            HierarchyEvent::ChildAdded { child, parent } if csg_nodes.contains(parent) => {
                h_ui_event.send(CsgHierarchyEvent::ChildAdded {
                    child: *child,
                    parent: *parent,
                });
            }
            HierarchyEvent::ChildRemoved { child, parent } if csg_nodes.contains(parent) => {
                h_ui_event.send(CsgHierarchyEvent::ChildRemoved {
                    child: *child,
                    old_parent: *parent,
                });
            }
            HierarchyEvent::ChildMoved {
                child,
                previous_parent,
                new_parent,
            } => {
                match (
                    csg_nodes.contains(previous_parent),
                    csg_nodes.contains(new_parent),
                ) {
                    (true, true) => {
                        h_ui_event.send(CsgHierarchyEvent::ChildMoved {
                            child: *child,
                            old_parent: *previous_parent,
                            new_parent: *new_parent,
                        });
                    }
                    (false, true) => {
                        h_ui_event.send(CsgHierarchyEvent::ChildAdded {
                            child: *child,
                            parent: *new_parent,
                        });
                    }
                    (true, false) => {
                        h_ui_event.send(CsgHierarchyEvent::ChildRemoved {
                            child: *child,
                            old_parent: *previous_parent,
                        });
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}
