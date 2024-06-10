use crate::prelude::*;
use std::marker::PhantomData;

use bevy::input::{mouse::MouseMotion, InputSystem};
use bevy::window::PrimaryWindow;

#[derive(Debug, PartialEq, Eq, Clone, Hash, SystemSet)]
pub struct InputCollectionSytem;

pub struct InputPlugin;
impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MovAxis3Settings>();
        app.init_resource::<MovAxis2Settings>();
        app.init_resource::<AxisSettings>();
        app.init_resource::<MouseSettings>();
        app.add_systems(
            PreUpdate,
            (
                update_mouse,
                update_mov3,
                update_mov2,
                update_axis,
                on_mouse_added,
            )
                .in_set(InputCollectionSytem)
                .after(InputSystem),
        );
    }
}

#[derive(Component, Default)]
pub struct DontUpdate<I: Component>(PhantomData<I>);

pub fn dont_update<I: Component + Default>() -> DontUpdate<I> {
    DontUpdate::default()
}

#[derive(Resource, Clone, Copy, Reflect, Component)]
pub struct MouseSettings {
    pub h_sens: f32,
    pub v_sens: f32,
    pub clamp_y: bool,
}

impl MouseSettings {
    pub fn as_vec2(&self) -> Vec2 {
        Vec2 {
            x: self.h_sens,
            y: self.v_sens,
        }
    }
}
impl From<MouseSettings> for Vec2 {
    fn from(val: MouseSettings) -> Self {
        Vec2 {
            x: val.h_sens,
            y: val.v_sens,
        }
    }
}

impl Default for MouseSettings {
    fn default() -> Self {
        Self {
            h_sens: 1f32,
            v_sens: 1f32,
            clamp_y: true,
        }
    }
}
// Mouse Component Tracking.
#[derive(Component, Default, Clone, Copy)]
pub struct Mouse {
    pub total: Vec2,
    pub delta: Vec2,
}

impl Mouse {
    pub fn update_total_from_tranform(&mut self, transform: &Transform) {
        let lx = *transform.local_x();
        let ly = *transform.local_y();
        self.total = Vec2::new(
            lx.signed_angle_in_plane(Vec3::Y, Vec3::X),
            ly.signed_angle_in_plane(lx, Vec3::Y),
        );
    }

    pub fn from_transform(transform: &Transform) -> Self {
        let mut mouse = Self::default();
        mouse.update_total_from_tranform(transform);
        mouse
    }

    pub fn apply_mouse_rot(&self, transform: &mut Transform) {
        transform.rotation = self.yaw() * self.pitch();
    }

    pub fn pitch(&self) -> Quat {
        Quat::from_rotation_x(-self.total.y)
    }

    pub fn yaw(&self) -> Quat {
        Quat::from_rotation_y(-self.total.x)
    }

    pub fn pan_amount(&self, transform: &mut Transform, scaled_speed: f32) {
        let pan_amount = self.delta * scaled_speed;
        transform.translation += pan_amount.y * *transform.up() - pan_amount.x * *transform.right()
    }
}

pub fn update_mouse(
    window: Query<&Window, With<PrimaryWindow>>,
    settings: Res<MouseSettings>,
    mut mouse_motions: EventReader<MouseMotion>,
    mut query: Query<&mut Mouse, Without<DontUpdate<Mouse>>>,
) {
    let window = window.single();
    let total_motion: Vec2 = mouse_motions.read().fold(Vec2::ZERO, |a, b| a + b.delta)
        * settings.as_vec2()
        / Vec2::new(window.width(), window.height());
    query.iter_mut().for_each(|mut mouse| {
        mouse.total += total_motion;
        if settings.clamp_y {
            mouse.total.y = mouse.total.y.clamp(-FRAC_PI_2, FRAC_PI_2);
        }
        mouse.delta = total_motion;
    });
}

pub fn on_mouse_added(
    mut mouse_input: Query<(&mut Mouse, &Transform), Added<Mouse>>,
) {
    for (mut mouse, transform) in &mut mouse_input {
        mouse.update_total_from_tranform(transform);
    }
}

#[derive(Component, Resource)]
pub struct MovAxis3Settings {
    forward: (KeyCode, KeyCode),
    lateral: (KeyCode, KeyCode),
    vertical: (KeyCode, KeyCode),
}

impl Default for MovAxis3Settings {
    fn default() -> Self {
        Self {
            forward: (KeyCode::KeyD, KeyCode::KeyS),
            lateral: (KeyCode::KeyA, KeyCode::KeyH),
            vertical: (KeyCode::Space, KeyCode::KeyQ),
        }
    }
}

#[derive(Component, Resource)]
pub struct AxisSettings(pub KeyCode, pub KeyCode);

impl Default for AxisSettings {
    fn default() -> Self {
        Self(KeyCode::Space, KeyCode::KeyQ)
    }
}

#[derive(Component, Default)]
pub struct Axis {
    pub pos: f32,
    pub neg: f32,
}

impl Axis {
    pub fn value(&self) -> f32 {
        self.pos - self.neg
    }

    pub fn is_pos_pressed(&self) -> bool {
        self.pos != 0.0
    }

    pub fn is_neg_pressed(&self) -> bool {
        self.neg != 0.0
    }
}

pub fn update_axis(
    settings: Res<AxisSettings>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Axis, Option<&AxisSettings>), Without<DontUpdate<MovAxis3>>>,
) {
    query.iter_mut().for_each(|(mut axis, _settings)| {
        let settings = _settings.unwrap_or(&settings);
        *axis = keyboard_input.axis(settings.0, settings.1);
    });
}

#[derive(Component, Default)]
pub struct MovAxis3 {
    pub forward: Axis,
    pub lateral: Axis,
    pub vertical: Axis,
}

impl MovAxis3 {
    pub fn horizontal(&self) -> Vec3 {
        self.forward.value() * Vec3::NEG_Z + self.lateral.value() * Vec3::NEG_X
    }

    pub fn horizontal_in_local(&self, local: &Transform) -> Vec3 {
        local.rotation * self.horizontal()
    }

    pub fn vertical(&self) -> Vec3 {
        self.vertical.value() * Vec3::Y
    }

    pub fn vertical_in_local(&self, local: &Transform) -> Vec3 {
        local.rotation * self.vertical()
    }

    pub fn total_movement(&self) -> Vec3 {
        self.horizontal() + self.vertical()
    }

    pub fn total_movement_in_local(&self, local: &Transform) -> Vec3 {
        local.rotation * (self.horizontal() + self.vertical())
    }

    pub fn plane_restricted_in_local(&self, local: &Transform, normal: Vec3) -> Vec3 {
        let dir = local.rotation * self.horizontal_in_local(local);
        let norm = dir.length();
        dir.reject_from(normal).normalize() * norm
    }
}

pub fn update_mov3(
    settings: Res<MovAxis3Settings>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut MovAxis3, Option<&MovAxis3Settings>), Without<DontUpdate<MovAxis3>>>,
) {
    query.iter_mut().for_each(|(mut mov, _settings)| {
        let settings = _settings.unwrap_or(&settings);
        mov.forward = keyboard_input.axis(settings.forward.0, settings.forward.1);
        mov.lateral = keyboard_input.axis(settings.lateral.0, settings.lateral.1);
        mov.vertical = keyboard_input.axis(settings.vertical.0, settings.vertical.1);
    });
}

#[derive(Component, Resource)]
pub struct MovAxis2Settings {
    forward: (KeyCode, KeyCode),
    lateral: (KeyCode, KeyCode),
}

impl Default for MovAxis2Settings {
    fn default() -> Self {
        Self {
            forward: (KeyCode::KeyD, KeyCode::KeyS),
            lateral: (KeyCode::KeyA, KeyCode::KeyH),
        }
    }
}

#[derive(Component, Default)]
pub struct MovAxis2 {
    forward: Axis,
    lateral: Axis,
}

impl MovAxis2 {
    pub fn movement_3d(&self) -> Vec3 {
        self.forward.value() * Vec3::NEG_Z + self.lateral.value() * Vec3::NEG_X
    }

    pub fn movement_3d_in_local(&self, local: &Transform) -> Vec3 {
        local.rotation * self.movement_3d()
    }
}

pub fn update_mov2(
    settings: Res<MovAxis2Settings>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut MovAxis2, Option<&MovAxis2Settings>), Without<DontUpdate<MovAxis2>>>,
) {
    query.iter_mut().for_each(|(mut mov, _settings)| {
        let settings = _settings.unwrap_or(&settings);
        mov.forward = keyboard_input.axis(settings.forward.0, settings.forward.1);
        mov.lateral = keyboard_input.axis(settings.lateral.0, settings.lateral.1);
    });
}
