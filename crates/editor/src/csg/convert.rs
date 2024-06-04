use crate::prelude::*;
use bevy::math::primitives::*;

use super::{BrushMesh, Plane};

pub trait Brushable: Sized {
    type Settings: Default;

    fn to_default_brush(self) -> BrushMesh {
        self.to_brush(Self::Settings::default())
    }

    fn to_brush(self, settings: Self::Settings) -> BrushMesh;
}

impl Brushable for Cuboid {
    type Settings = ();

    fn to_brush(self, _settings: Self::Settings) -> BrushMesh {
        let Cuboid { half_size } = self;
        let mut brush = BrushMesh::empty();
        let corners = brush.add_vertices([
            // Top Face.
            half_size,
            half_size.flip_z(),
            half_size.flip_xz(),
            half_size.flip_x(),
            // Bottom Face.
            half_size.flip_y(),
            half_size.flip_yz(),
            half_size.flip(),
            half_size.flip_xy(),
        ]);

        let planes = brush.add_planes([
            Plane::new(half_size.x * Vec3::X, Vec3::X),
            Plane::new(half_size.x * Vec3::NEG_X, Vec3::NEG_X),
            Plane::new(half_size.y * Vec3::Y, Vec3::Y),
            Plane::new(half_size.y * Vec3::NEG_Y, Vec3::NEG_Y),
            Plane::new(half_size.z * Vec3::Z, Vec3::Z),
            Plane::new(half_size.z * Vec3::NEG_Z, Vec3::NEG_Z),
        ]);

        brush.add_polygons([
            &[corners[0], corners[1], corners[2], corners[3]],
            &[corners[4], corners[7], corners[6], corners[5]],
            &[corners[0], corners[4], corners[5], corners[1]],
            &[corners[3], corners[2], corners[6], corners[7]],
            &[corners[0], corners[3], corners[7], corners[4]],
            &[corners[1], corners[5], corners[6], corners[2]],
        ]);

        brush
    }
}

pub struct Plane3dBrushMeshSettings {
    size: f32,
}

impl Default for Plane3dBrushMeshSettings {
    fn default() -> Self {
        Self { size: 10f32 }
    }
}

impl Brushable for Plane3d {
    type Settings = Plane3dBrushMeshSettings;
    fn to_brush(self, settings: Self::Settings) -> BrushMesh {
        let Self::Settings { size } = settings;
        let local_y = self.normal;
        let (local_x, local_z) = local_y.any_orthonormal_pair();
        let mut brush = BrushMesh::empty();
        let corners = brush.add_vertices([
            local_x * size + local_z * size,
            -local_x * size + local_z * size,
            -local_x * size - local_z * size,
            local_x * size - local_z * size,
        ]);
        brush.add_polygon(&corners);
        brush
    }
}
