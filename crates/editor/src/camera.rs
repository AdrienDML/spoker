use bevy::{
    input::mouse::MouseWheel,
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};
use common::input;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_flycam_speed,
                (manage_flycam, update_flycam).chain(),
                (manage_pancam, update_pancam).chain(),
            ),
        );
    }
}

#[derive(Component)]
pub struct MainCam;

#[derive(Component)]
pub struct Pan;

#[derive(Component)]
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
    query: Query<Entity, (With<MainCam>, Without<FlyCam>)>,
    mut window: Query<&mut Window, With<PrimaryWindow>>,
    mut commands: Commands,
) {
    let Ok(entity) = query.get_single() else {
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
    } else if input.just_released(MouseButton::Middle) {
        cursor.grab_mode = CursorGrabMode::None;
        cursor.visible = true;
        commands
            .entity(entity)
            .remove::<Pan>()
            .insert(input::DontUpdate::<input::Mouse>::default());
    }
}

pub fn manage_flycam(
    input: Res<ButtonInput<MouseButton>>,
    query: Query<Entity, (With<MainCam>, Without<Pan>)>,
    mut commands: Commands,
    mut window: Query<&mut Window, With<PrimaryWindow>>,
) {
    let Ok(entity) = query.get_single() else {
        return;
    };
    let cursor = &mut window.single_mut().cursor;
    if input.just_pressed(MouseButton::Right) {
        cursor.grab_mode = CursorGrabMode::Locked;
        cursor.visible = false;
        commands
            .entity(entity)
            .insert(FlyCam::default())
            .remove::<input::DontUpdate<input::Mouse>>()
            .remove::<input::DontUpdate<input::MovAxis3>>();
    } else if input.just_released(MouseButton::Right) {
        cursor.grab_mode = CursorGrabMode::None;
        cursor.visible = true;
        commands
            .entity(entity)
            .remove::<FlyCam>()
            .insert(input::DontUpdate::<input::Mouse>::default())
            .insert(input::DontUpdate::<input::MovAxis3>::default());
    }
}

pub fn update_pancam(
    mut query: Query<(&mut Transform, &input::Mouse), (With<MainCam>, With<Pan>)>,
) {
    for (mut cam_transform, mouse) in &mut query {
        mouse.pan_amount(&mut cam_transform)
    }
}

pub fn update_flycam(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &FlyCam, &input::Mouse, &input::MovAxis3), With<MainCam>>,
) {
    let Ok((mut cam_transform, fly_cam, mouse, mov)) = query.get_single_mut() else {
        return;
    };
    let scaled_speed = fly_cam.speed * time.delta_seconds();
    cam_transform.rotation = mouse.yaw() * mouse.pitch();
    mov.apply_horizontal_in_local(&mut cam_transform, scaled_speed);
    mov.apply_vertical(&mut cam_transform, scaled_speed);
}

pub fn update_flycam_speed(
    mut mouse_wheel: EventReader<MouseWheel>,
    mut query: Query<&mut FlyCam, With<MainCam>>,
) {
    let wheel_motion = mouse_wheel.read().fold(0f32, |tot, wheel| tot + wheel.y);
    for mut fly_cam in &mut query {
        fly_cam.speed += wheel_motion;
        fly_cam.speed = fly_cam.speed.max(0f32);
    }
}
