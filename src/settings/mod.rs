pub mod movement;
pub mod player;

//pub struct SettingPlugin<S: Setting>(PhantomData<S>);
//
//impl<S: Setting> Plugin for SettingPlugin<S> {
//    fn build(&self, app: &mut App) {
//        app.init_resource::<S>()
//        .add_systems(PreUpdate, update_on_change::<S>);
//    }
//}
//
//pub trait Setting: Resource + Default {
//    type Data<'w>: QueryData;
//    type Filter<'w>: QueryFilter;
//
//    fn update_on_change<'w>(&self, data: <Self::Data<'w> as WorldQuery>::Item<'w>);
//}
//
//
//fn update_on_change<'w, S: Setting>(
//    setting: Res<S>,
//    mut query: Query<S::Data<'w>, S::Filter<'w>>,
//) {
//    if !setting.is_changed() {
//        return;
//    }
//
//    for data in &mut query {
//        setting.update_on_change(data)
//    }
//}
