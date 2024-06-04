use std::marker::PhantomData;

use crate::prelude::*;

pub trait Setting: Component + Resource + Default {
    type Property: Component + Resource;

    fn setting(&self) -> Self::Property;
}

pub fn track_setting<S: Setting>(
    mut commands: Commands,
    prop: Res<S>,
    mut setting: ResMut<S::Property>,
    mut update_entity_overides: Query<(&S, &mut S::Property), Changed<S>>,
    new_entity_overides: Query<(Entity, &S), Added<S>>,
) {
    if prop.is_changed() {
        *setting = prop.setting();
    }

    for (prop, mut setting) in &mut update_entity_overides {
        *setting = prop.setting();
    }
    for (entity, prop) in &new_entity_overides {
        commands.entity(entity).insert(prop.setting());
    }
}
#[derive(Default)]
pub struct SettingPlugin<S: Setting>(PhantomData<S>);

impl<S:Setting> Plugin for SettingPlugin<S> {
    fn build(&self, app: &mut App) {
        let prop = S::default();
        app.add_systems(PreUpdate, track_setting::<S>)
            .insert_resource(prop.setting())
            .insert_resource(prop);
    }
}
