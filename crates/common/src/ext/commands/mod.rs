use crate::prelude::*;


pub trait CommandExt {
    fn despawn_child(&mut self, parent: Entity, child: Entity);

    fn despawn_childrens(&mut self, parent: Entity, children: &[Entity]);

    fn move_child(&mut self, current_parent: Entity, new_parent: Entity, child: Entity);
}

impl CommandExt for Commands<'_, '_> {
    fn despawn_child(&mut self, parent: Entity, child: Entity) {
        self.entity(parent).remove_children(&[child]);
        self.entity(child).despawn()
    }

    fn despawn_childrens(&mut self, parent: Entity, children: &[Entity]) {
        self.entity(parent).remove_children(children);
        for child in children {
            self.entity(*child).despawn()
        }
    }

    fn move_child(&mut self, current_parent: Entity, new_parent: Entity, child: Entity) {
        self.entity(current_parent).remove_children(&[child]);
        self.entity(new_parent).add_child(child);
    }
}
