use crate::prelude::*;
pub use bevy_rapier3d::*;
use prelude::*;

pub mod col_layers {
    use super::geometry::Group;

    // What can colide with players.
    pub const PLAYERS: Group = Group::GROUP_1;

    // What can colide with Shots.
    pub const HURTBOXES: Group = Group::GROUP_2;

    // What can colide with players.
    pub const HITBOXES: Group = Group::GROUP_3;

    // What can collide
    pub const ENVIRONEMENT: Group = Group::GROUP_4;
}

pub struct PhysicsPlugin;

use plugin::NoUserData;
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
        RapierPhysicsPlugin::<NoUserData>::default()
            .in_schedule(FixedUpdate),
        );
        let dt = 1.0 / 60.0;
        app.insert_resource(Time::<Fixed>::from_hz(60.0));
        app.insert_resource(RapierConfiguration {
            timestep_mode: TimestepMode::Fixed { dt, substeps: 1 },
            ..default()
        });
    }
}

#[derive(Clone, Copy)]
pub struct CollisionSettings {
    // Should be small.
    pub skin_width: f32,
    pub collision_group: CollisionGroups,
}

pub struct EntityData<'e> {
    pub entity: Entity,
    pub collider: &'e Collider,
    pub transform: &'e Transform,
    pub velocity: Vec3,
    pub grounded: bool,
}

pub fn move_and_collide(
    physics_context: &RapierContext,
    collision_settings: CollisionSettings,
    entity: Entity,
    collider: &Collider,
    transform: &mut Transform,
    dir: Vec3,
    speed: f32,
) -> Option<ShapeHit> {
    if let Some(hit) = cast_shape(
        physics_context,
        collision_settings,
        entity,
        collider,
        transform,
        dir,
        speed,
    ) {
        trace!("CastShape hit:");
        trace!("{hit:?}");
        transform.translation += hit.toi * dir;
        Some(hit)
    } else {
        trace!("CastShape Missed.");
        None
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ShapeHit {
    pub toi: f32,
    pub point: Vec3,
    pub normal: Vec3,
}

pub fn cast_shape(
    physics_context: &RapierContext,
    collision_settings: CollisionSettings,
    entity: Entity,
    collider: &Collider,
    transform: &Transform,
    dir: Vec3,
    max_dist: f32,
) -> Option<ShapeHit> {
    let CollisionSettings {
        skin_width,
        collision_group,
    } = collision_settings;
    // Avoid self collisions.
    let filter = QueryFilter::new()
        .groups(collision_group)
        .exclude_rigid_body(entity);
    trace!("Cast Shape:");
    trace!("\t dir: {dir:?}");
    trace!("\t max_dist: {max_dist}");
    trace!("\t pos: {}", transform.translation);
    let (_, hit) = physics_context.cast_shape(
        transform.translation,
        transform.rotation,
        dir,
        collider,
        max_dist,
        true,
        filter,
    )?;
    trace!("Cast Shape hit: {hit:?}");
    let details = hit.details?;
    Some(ShapeHit {
        toi: hit.toi - skin_width,
        point: details.witness1,
        normal: details.normal1,
    })
}
