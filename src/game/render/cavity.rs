use bevy::render::render_resource::AsBindGroup;

use crate::prelude::*;

#[derive(Asset, AsBindGroup, TypePath, Debug, Clone)]
pub struct CavityMaterial {
    curvature_scale: f32,
    curvature_ridge: f32,
    curvature_valey: f32,
    cavity_distance: f32,
    cavity_attenuation: f32,
    cavity_ridge: f32,
    cavity_valley: f32,
    cavity_samples: u32,
}

impl Default for CavityMaterial {
    fn default() -> Self {
        Self {
            curvature_scale: 1.0,
            curvature_ridge: 0.25,
            curvature_valey: 0.25,
            cavity_distance: 0.25,
            cavity_attenuation: 0.015625,
            cavity_ridge: 1.25,
            cavity_valley: 1.25,
            cavity_samples: 4,
        }
    }
}

impl Material for CavityMaterial {
    fn fragment_shader() -> bevy::render::render_resource::ShaderRef {
        "shaders/cavity.wgsl".into()
    }
}
