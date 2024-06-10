use crate::prelude::*;

use super::super::csg::{hierarchy::CsgHierarchyEvent, CsgRoot, CsgLeaf, CsgNode};

#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct HierarchyView;

#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct HierarchyViewContainer;

/// Component on a csg ui node pointing to the coresponding entity in the csg tree.
#[derive(Component)]
pub struct CsgRefNode(Entity);

#[derive(Event)]
pub struct RefreshHierarchyView;

pub fn setup(mut commands: Commands, h_ui_root: Query<Entity, With<HierarchyView>>) {
    let h_ui_root = h_ui_root.single();
    commands.ui_builder(h_ui_root).column(|col| {
        col.row(|row| {
            row.style()
                .height(val!(30.0 px))
                .padding(UiRect::all(val!(3.0 px)))
                .background_color(Color::GRAY);
            row.label(LabelConfig {
                label: "TreeView".to_string(),
                ..default()
            });
            row.button("Refresh");
        });
        col.menu_item_separator();
        col.column(|col| {
            col.insert(HierarchyViewContainer);
        });
    });
}

pub fn on_h_ui_refresh(
    mut commands: Commands,
    mut e_refresh: EventReader<RefreshHierarchyView>,
    h_ui_cont: Query<Entity, With<HierarchyViewContainer>>,
    csg_root: Query<&Children, With<CsgRoot>>,
    csg_nodes: Query<(Entity, &Name, &Children), With<CsgNode>>,
    csg_leafs: Query<(Entity, &Name), With<CsgLeaf>>,
) {
    for _ in e_refresh.read() {
        let mut ui_parent = commands.ui_builder(h_ui_cont.single());
        let csg_children = csg_root.single();
        spawn_ui_nodes_rec(&mut ui_parent, csg_children, &csg_nodes, &csg_leafs);
    }
}

fn spawn_ui_nodes_rec(
    ui_parent: &mut UiBuilder<Entity>,
    csg_children: &Children,
    csg_nodes: &Query<(Entity, &Name, &Children), With<CsgNode>>,
    csg_leafs: &Query<(Entity, &Name), With<CsgLeaf>>,
) {
    for child in csg_children {
        if let Ok((entity, name, csg_children)) = csg_nodes.get(*child) {
            ui_parent.foldable(name.to_string(), true, |child| {
                child.insert(CsgRefNode(entity));
                spawn_ui_nodes_rec(child, csg_children, csg_nodes, csg_leafs);
            });
        } else if let Ok((entity, name)) = csg_leafs.get(*child) {
            ui_parent
                .label(LabelConfig {
                    label: name.to_string(),
                    ..default()
                })
                .insert(CsgRefNode(entity));
        } else {
            continue;
        }
    }
}

/// Must be ran after `setup_csg_root`.
pub fn set_up_csg_root_ui_link(
    mut commands: Commands,
    h_csg_root: Query<Entity, With<CsgRoot>>,
    h_ui_root: Query<Entity, With<HierarchyView>>,
) {
    let Ok(csg_root) = h_csg_root.get_single() else {
        error!("Multiple csg roots are found.");
        return;
    };

    let Ok(csg_ui_root) = h_ui_root.get_single() else {
        error!("Multiple ui roots are found for the hierarchy view.");
        return;
    };

    commands
        .ui_builder(csg_ui_root)
        .insert(CsgRefNode(csg_root));
}

pub fn update_hierarchy_ui(
    mut commands: Commands,
    mut h_events: EventReader<CsgHierarchyEvent>,
    h_csg_nodes: Query<&Name, With<CsgNode>>,
    h_ui_nodes: Query<(Entity, &CsgRefNode)>,
) {
    for h_event in h_events.read() {
        match h_event {
            CsgHierarchyEvent::ChildAdded { child, parent } => {
                let Some(parent_ui_node) = get_h_ui_node(&h_ui_nodes, *parent) else {
                    error!("A csg node doesn't have a coresponding ui node.");
                    continue;
                };

                let Ok(label) = h_csg_nodes.get(*child) else {
                    error!("The children csg node doesn't have a name");
                    continue;
                };

                commands.ui_builder(parent_ui_node).label(LabelConfig {
                    label: label.to_string(),
                    ..default()
                });
            }
            CsgHierarchyEvent::ChildRemoved { child, old_parent } => {
                let Some(child_ui_node) = get_h_ui_node(&h_ui_nodes, *child) else {
                    error!("A csg node doesn't have a coresponding ui node.");
                    continue;
                };
                let Some(paren_ui_node) = get_h_ui_node(&h_ui_nodes, *old_parent) else {
                    error!("A csg node doesn't have a coresponding ui node.");
                    continue;
                };

                commands.despawn_child(paren_ui_node, child_ui_node);
            }
            CsgHierarchyEvent::ChildMoved {
                child,
                old_parent,
                new_parent,
            } => {
                let Some(child_ui_node) = get_h_ui_node(&h_ui_nodes, *child) else {
                    error!("A csg node doesn't have a coresponding ui node.");
                    continue;
                };
                let Some(old_parent_ui_node) = get_h_ui_node(&h_ui_nodes, *old_parent) else {
                    error!("A csg node doesn't have a coresponding ui node.");
                    continue;
                };
                let Some(new_parent_ui_node) = get_h_ui_node(&h_ui_nodes, *new_parent) else {
                    error!("A csg node doesn't have a coresponding ui node.");
                    continue;
                };
                commands.move_child(old_parent_ui_node, new_parent_ui_node, child_ui_node)
            }
        }
    }
}

fn get_h_ui_node(h_ui_nodes: &Query<(Entity, &CsgRefNode)>, h_csg_node: Entity) -> Option<Entity> {
    h_ui_nodes.iter().find_map(|(entity, csg_ref)| {
        if csg_ref.0 == h_csg_node {
            Some(entity)
        } else {
            None
        }
    })
}
