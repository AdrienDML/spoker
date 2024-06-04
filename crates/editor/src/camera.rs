use bevy::{
    input::mouse::MouseWheel,
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};
use common::input;

use crate::RunOnMapFocused;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_flycam_speed,
                (manage_flycam, update_flycam).chain(),
                (manage_pancam, update_pancam).chain(),
            ).in_set(RunOnMapFocused),
        );
    }
}

#[derive(Component)]
pub struct UiCamera;

#[derive(Component)]
pub struct MapCamera;

#[derive(Component)]
pub struct Pan;

#[derive(Component, Clone, Copy)]
pub struct FlyCam {
    speed: f32,
}

impl Default for FlyCam {
    fn default() -> Self {
        Self { speed: 7.0 }
    }
}

pub fn manage_pancam(
    input: Res<ButtonInput<MouseButton>>,
    mut query: Query<(Entity, &mut input::Mouse), (With<MapCamera>, Without<FlyCam>)>,
    mut window: Query<&mut Window, With<PrimaryWindow>>,
    mut commands: Commands,
    mut mouse_cached: Local<input::Mouse>,
) {
    let Ok((entity, mut mouse)) = query.get_single_mut() else {
        return;
    };
    let cursor = &mut window.single_mut().cursor;
    if input.just_pressed(MouseButton::Middle) {
        cursor.grab_mode = CursorGrabMode::Locked;
        cursor.visible = false;
        commands
            .entity(entity)
            .insert(Pan)
            .remove::<input::DontUpdate<input::Mouse>>();
        *mouse = *mouse_cached;
    } else if input.just_released(MouseButton::Middle) {
        cursor.grab_mode = CursorGrabMode::None;
        cursor.visible = true;
        commands
            .entity(entity)
            .remove::<Pan>()
            .insert(input::DontUpdate::<input::Mouse>::default());
        *mouse_cached = *mouse;
    }
}

pub fn manage_flycam(
    input: Res<ButtonInput<MouseButton>>,
    mut query: Query<(Entity, &mut input::Mouse, Option<&FlyCam>), (With<MapCamera>, Without<Pan>)>,
    mut commands: Commands,
    mut window: Query<&mut Window, With<PrimaryWindow>>,
    mut cached: Local<(input::Mouse, FlyCam)>
) {
    let Ok((entity, mut mouse, flycam)) = query.get_single_mut() else {
        return;
    };
    let cursor = &mut window.single_mut().cursor;
    if input.just_pressed(MouseButton::Right) {
        cursor.grab_mode = CursorGrabMode::Locked;
        cursor.visible = false;
        commands
            .entity(entity)
            .insert(cached.1)
            .remove::<input::DontUpdate<input::Mouse>>()
            .remove::<input::DontUpdate<input::MovAxis3>>();
        *mouse = cached.0;
    } else if input.just_released(MouseButton::Right) {
        cursor.grab_mode = CursorGrabMode::None;
        cursor.visible = true;
        
        commands
            .entity(entity)
            .remove::<FlyCam>()
            .insert(input::DontUpdate::<input::Mouse>::default())
            .insert(input::DontUpdate::<input::MovAxis3>::default());

        // Fly cam should be present.
        *cached = (*mouse, *flycam.unwrap());
    }
}

pub fn update_pancam(
    mut query: Query<(&mut Transform, &input::Mouse), (With<MapCamera>, With<Pan>)>,
) {
    for (mut cam_transform, mouse) in &mut query {
        mouse.pan_amount(&mut cam_transform)
    }
}

pub fn update_flycam(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &FlyCam, &input::Mouse, &input::MovAxis3), With<MapCamera>>,
) {
    let Ok((mut cam_transform, fly_cam, mouse, mov)) = query.get_single_mut() else {
        return;
    };
    let scaled_speed = fly_cam.speed * time.delta_seconds();
    let translation = (mov.horizontal_in_local(&cam_transform) +
        mov.vertical()) * scaled_speed;
    cam_transform.translation += translation;
    cam_transform.rotation = mouse.yaw() * mouse.pitch();
}

pub fn update_flycam_speed(
    mut mouse_wheel: EventReader<MouseWheel>,
    mut query: Query<&mut FlyCam, With<MapCamera>>,
) {
    let wheel_motion = mouse_wheel.read().fold(0f32, |tot, wheel| tot + wheel.y);
    for mut fly_cam in &mut query {
        fly_cam.speed += wheel_motion;
        fly_cam.speed = fly_cam.speed.max(0f32);
    }
}
