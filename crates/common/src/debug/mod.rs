use crate::prelude::*;
use std::{fmt, marker::PhantomData};

use bevy::ecs::query::{QueryData, QueryFilter, WorldQuery};

pub struct DebugComponentPlugin<C, F>(PhantomData<(C, F)>)
where
    C: QueryData + Send + Sync,
    F: QueryFilter + Send + Sync,
    for<'a> Item<'a, C>: fmt::Debug;

impl<C, F> Default for DebugComponentPlugin<C, F>
where
    C: QueryData + Send + Sync,
    F: QueryFilter + Send + Sync + 'static,
    for<'a> Item<'a, C>: fmt::Debug,
{
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<C, F> Plugin for DebugComponentPlugin<C, F>
where
    Self: 'static,
    C: QueryData + Send + Sync + 'static,
    F: QueryFilter + Send + Sync + 'static,
    for<'a> Item<'a, C>: fmt::Debug,
{
    fn build(&self, app: &mut App) {
        app.add_systems(Last, trace_component::<C, F>);
    }
}
#[derive(Component)]
pub struct DebugComponent;

pub type Item<'a, C> = <<C as QueryData>::ReadOnly as WorldQuery>::Item<'a>;

pub fn trace_component<'w, 's, C, F>(trace_component: Query<'w, 's, (Option<&Name>, C), F>)
where
    C: QueryData,
    F: QueryFilter,
    for<'a> Item<'a, C>: fmt::Debug,
{
    for (name, comp) in &trace_component {
        if let Some(name) = name {
            debug!("{name} = {comp:#?}")
        } else {
            debug!("{comp:#?}")
        }
    }
}
