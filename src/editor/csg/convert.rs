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

impl<B> From<B> for BrushMesh where B: Brushable {
    fn from(value: B) -> Self {
        value.to_default_brush()
    }
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
            Plane::new(half_size.x * Vec3::X, Vec3::X), // 0 left
            Plane::new(half_size.x * Vec3::NEG_X, Vec3::NEG_X), // 1 right
            Plane::new(half_size.y * Vec3::Y, Vec3::Y), // Top 2
            Plane::new(half_size.y * Vec3::NEG_Y, Vec3::NEG_Y), // Bottom 3
            Plane::new(half_size.z * Vec3::Z, Vec3::Z), // front
            Plane::new(half_size.z * Vec3::NEG_Z, Vec3::NEG_Z), // back
        ]);

        brush.add_polygon_on_plane(&[corners[0], corners[4], corners[5], corners[1]], planes[0]);
        brush.add_polygon_on_plane(&[corners[3], corners[2], corners[6], corners[7]], planes[1]);
        brush.add_polygon_on_plane(&[corners[0], corners[1], corners[2], corners[3]], planes[2]);
        brush.add_polygon_on_plane(&[corners[4], corners[7], corners[6], corners[5]], planes[3]);
        brush.add_polygon_on_plane(&[corners[0], corners[3], corners[7], corners[4]], planes[4]);
        brush.add_polygon_on_plane(&[corners[1], corners[5], corners[6], corners[2]], planes[5]);

        brush
    }
}

pub struct Plane3dBrushMeshSettings {
    size: f32,
    point: Vec3,
}

impl Default for Plane3dBrushMeshSettings {
    fn default() -> Self {
        Self {
            size: 10f32,
            point: Vec3::ZERO,
        }
    }
}

impl Brushable for Plane3d {
    type Settings = Plane3dBrushMeshSettings;
    fn to_brush(self, settings: Self::Settings) -> BrushMesh {
        let Self::Settings { size, point } = settings;
        let local_y = self.normal;
        let (local_x, local_z) = local_y.any_orthonormal_pair();
        let mut brush = BrushMesh::empty();
        let corners = brush.add_vertices([
            local_x * size + local_z * size,
            -local_x * size + local_z * size,
            -local_x * size - local_z * size,
            local_x * size - local_z * size,
        ]);
        let plane = brush.add_plane(*self.normal, point);
        brush.add_polygon_on_plane(&corners, plane);
        brush
    }
}

pub struct Slope {
    pub length: f32,
    pub width: f32,
    pub height: f32,
}

impl From<Slope> for BrushMesh {
    fn from(val: Slope) -> Self {
        let mut brush = BrushMesh::empty();
        let half = Vec3::new(val.width * 0.5, 0.0, val.length * 0.5);

        brush.add_vertices([
            half,
            half.flip_x(),
            half.flip_xz(),
            half.flip_z(),
            half.flip_xz() + Vec3::Y * val.height,
            half.flip_z() + Vec3::Y * val.height,
        ]);
        let bottom = brush.add_plane(Vec3::NEG_Y, Vec3::ZERO);
        let slope = brush.add_plane(
            Vec3::new(0.0, val.length, val.height).normalize(),
            Vec3::new(0.0, 0.5 * val.height, 0.0),
        );
        let back = brush.add_plane(
            Vec3::NEG_Z,
            Vec3::new(0.0, 0.5 * val.height, -val.length * 0.5),
        );
        let side = brush.add_plane(
                    Vec3::X,
            Vec3::X * val.width * 0.5,
        );
        let side_neg = brush.add_plane(
                    Vec3::NEG_X,
            Vec3::NEG_X * val.width * 0.5,
        );

        brush.add_polygon_on_plane(&[0, 1, 2, 3], bottom);
        brush.add_polygon_on_plane(&[0, 5, 4, 1], slope);
        brush.add_polygon_on_plane(&[4, 5, 3, 2], back);
        brush.add_polygon_on_plane(&[0, 3, 5], side);
        brush.add_polygon_on_plane(&[1, 4, 2], side_neg);
        brush
    }
}
