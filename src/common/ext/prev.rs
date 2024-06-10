use bevy::prelude::*;
use std::{marker::PhantomData, mem, ops};

#[derive(Default)]
pub struct PrevPlugin<C: Component + Clone>(PhantomData<C>);

impl<C: Component + Clone> Plugin for PrevPlugin<C> {
    fn build(&self, app: &mut App) {
        app.add_systems(Last, store_prev_system::<C>);
    }
}

#[derive(Component)]
pub struct Prev<C: Component> {
    prev: Option<C>,
    curr: C,
}

impl<C> ops::Deref for Prev<C>
where
    C: Component + Clone,
{
    type Target = C;

    fn deref(&self) -> &Self::Target {
        self.prev.as_ref().unwrap_or(&self.curr)
    }
}

impl<C: Component + Default> Prev<C> {
    fn default(val: C) -> Self {
        Self {
            prev: Some(C::default()),
            curr: val,
        }
    }
}

impl<C: Component + Clone> Prev<C> {
    fn new(val: C) -> Self {
        Self {
            prev: None,
            curr: val,
        }
    }

    fn store_current_value(&mut self, val: &C) {
        self.prev = Some(mem::replace(&mut self.curr, val.clone()));
    }

    fn prev(&self) -> &C {
        self.prev.as_ref().unwrap_or(&self.curr)
    }
}

pub fn store_prev_system<C>(mut query: Query<(&C, &mut Prev<C>)>)
where
    C: Component + Clone,
{
    for (val, mut prev) in query.iter_mut() {
        prev.store_current_value(val);
    }
}
