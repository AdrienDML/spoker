use bevy::{
    input::mouse::MouseWheel,
    window::{CursorGrabMode, PrimaryWindow},
};
use crate::common::input;
use crate::prelude::*;

use super::RunOnMapFocused;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_cam_speed,
                (manage_flycam, update_flycam).chain(),
                (manage_pancam, update_pancam).chain(),
            )
                .in_set(RunOnMapFocused),
        );
    }
}

#[derive(Component)]
pub struct UiCamera;

#[derive(Component)]
pub struct ViewCamera {
    pub focused: bool,
}

#[derive(Component, Clone, Copy)]
pub struct Pan {
    speed: f32,
}

impl Default for Pan {
    fn default() -> Self {
        Self { speed: 14.0 }
    }
}

#[derive(Component, Clone, Copy)]
pub struct CanPan;

#[derive(Component, Clone, Copy)]
pub struct CanOrbit;

#[derive(Component, Clone, Copy)]
pub struct CanFly;

#[derive(Component, Clone, Copy)]
pub enum CamMode {
    Fly,
    Orbit,
    Pan,
}

pub fn manage_cam_mode(
    input: Res<ButtonInput<KeyCode>>,
    mut view_cams: Query<(
        &mut CamMode,
        &ViewCamera,
        Option<&CanPan>,
        Option<&CanFly>,
        Option<&CanOrbit>,
    )>,
) {
    for (mut mode, view, pan, fly, orbit) in &mut view_cams {
        if !view.focused {
            continue;
        }
        let can_pan = pan.is_some();
        let can_fly = fly.is_some();
        let can_orbit = orbit.is_some();

        if input.just_pressed(KeyCode::F1) && can_pan {
            *mode = CamMode::Pan;
        }
        if input.just_pressed(KeyCode::F2) && can_fly {
            *mode = CamMode::Fly;
        }
        if input.just_pressed(KeyCode::F3) && can_orbit {
            *mode = CamMode::Orbit;
        }
    }
}

#[derive(Component, Clone, Copy)]
pub struct CamSpeed(f32);

#[derive(Component, Clone, Copy)]
pub struct CamFocus(Vec3);

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
    input: Res<ButtonInput<KeyCode>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut query: Query<(Entity, Option<&Pan>), (With<ViewCamera>, With<CanPan>, Without<FlyCam>)>,
    mut window: Query<&mut Window, With<PrimaryWindow>>,
    mut commands: Commands,
    mut cached: Local<Pan>,
) {
    let Ok((entity, pan)) = query.get_single_mut() else {
        return;
    };
    let cursor = &mut window.single_mut().cursor;
    if mouse_input.just_pressed(MouseButton::Right) && input.pressed(KeyCode::Space) {
        cursor.grab_mode = CursorGrabMode::Locked;
        cursor.visible = false;
        commands
            .entity(entity)
            .insert(*cached)
            .remove::<input::DontUpdate<input::Mouse>>();
    } else if mouse_input.just_released(MouseButton::Right) {
        cursor.grab_mode = CursorGrabMode::None;
        cursor.visible = true;
        commands
            .entity(entity)
            .remove::<Pan>()
            .insert(input::DontUpdate::<input::Mouse>::default());
        // Pan is present here.
        if let Some(pan) = pan {
            *cached = *pan;
        }
    }
}

pub fn manage_flycam(
    input: Res<ButtonInput<KeyCode>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut query: Query<
        (Entity, &Transform, &mut input::Mouse, Option<&FlyCam>),
        (With<ViewCamera>, Without<Pan>, With<CanFly>),
    >,
    mut commands: Commands,
    mut window: Query<&mut Window, With<PrimaryWindow>>,
    mut cached: Local<FlyCam>,
) {
    let Ok((entity, transform, mut mouse, flycam)) = query.get_single_mut() else {
        return;
    };
    let cursor = &mut window.single_mut().cursor;
    if mouse_input.just_pressed(MouseButton::Right) && !input.pressed(KeyCode::Space) {
        cursor.grab_mode = CursorGrabMode::Locked;
        cursor.visible = false;
        mouse.update_total_from_tranform(transform);
        commands
            .entity(entity)
            .insert(*cached)
            .remove::<input::DontUpdate<input::Mouse>>()
            .remove::<input::DontUpdate<input::MovAxis3>>();
    } else if mouse_input.just_released(MouseButton::Right) {
        cursor.grab_mode = CursorGrabMode::None;
        cursor.visible = true;

        commands
            .entity(entity)
            .remove::<FlyCam>()
            .insert(input::DontUpdate::<input::Mouse>::default())
            .insert(input::DontUpdate::<input::MovAxis3>::default());

        // Fly cam should be present.
        if let Some(flycam) = flycam {
            *cached = *flycam;
        }
    }
}

pub fn update_pancam(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &input::Mouse, &Pan), With<ViewCamera>>,
) {
    for (mut cam_transform, mouse, pan) in &mut query {
        let scaled_speed = pan.speed * time.delta_seconds();
        mouse.pan_amount(&mut cam_transform, scaled_speed);
    }
}

pub fn update_flycam(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &FlyCam, &input::Mouse, &input::MovAxis3), With<ViewCamera>>,
) {
    let Ok((mut cam_transform, fly_cam, mouse, mov)) = query.get_single_mut() else {
        return;
    };
    let scaled_speed = fly_cam.speed * time.delta_seconds();
    let translation = (mov.horizontal_in_local(&cam_transform) + mov.vertical()) * scaled_speed;
    cam_transform.translation += translation;
    cam_transform.rotation = mouse.yaw() * mouse.pitch();
}

pub fn update_cam_speed(
    mut mouse_wheel: EventReader<MouseWheel>,
    mut query: Query<AnyOf<(&mut FlyCam, &mut Pan)>, With<ViewCamera>>,
) {
    let wheel_motion = mouse_wheel.read().fold(0f32, |tot, wheel| tot + wheel.y);
    for (fly_cam, pan) in &mut query {
        if let Some(mut cam) = fly_cam {
            cam.speed = (cam.speed + wheel_motion).max(0f32);
        }
        if let Some(mut cam) = pan {
            cam.speed = (cam.speed + wheel_motion).max(0f32);
        }
    }
}
