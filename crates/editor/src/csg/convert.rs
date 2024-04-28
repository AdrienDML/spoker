use bevy::math::primitives::*;
use crate::prelude::*;

use super::BrushMesh;

pub trait Brushable: Sized {
    type Settings: Default;

    fn to_default_brush(self) -> BrushMesh {
        self.to_brush(Self::Settings::default())
    } 

    fn to_brush(self, settings: Self::Settings) -> BrushMesh; 
}

pub struct CuboidBrushMeshSettings {
    subdivision: usize,
    project_on_sphere: bool,
}

impl Default for CuboidBrushMeshSettings {
    fn default() -> Self {
        Self {
            subdivision: 1,
            project_on_sphere: false,
        }
    }
}

impl Brushable for Cuboid {
    type Settings = CuboidBrushMeshSettings;


    fn to_brush(self, _settings: Self::Settings) -> BrushMesh {
        let h_size = self.half_size;
        let mut brush = BrushMesh::empty();
        let corners = brush.add_vertices([
            // Top Face.
            h_size,
            h_size.flip_z(),
            h_size.flip_xz(),
            h_size.flip_x(),
            // Bottom Face.
            h_size.flip_y(),
            h_size.flip_yz(),
            h_size.flip(),
            h_size.flip_xy(),
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
        Self {size: 10f32}
    }
}

impl Brushable for Plane3d {
    type Settings = Plane3dBrushMeshSettings;
    fn to_brush(self, settings: Self::Settings) -> BrushMesh {
        let size = settings.size;
        let local_y = self.normal;
        let (local_x, local_z) = local_y.any_orthonormal_pair();
        let mut brush = BrushMesh::empty();
        let corners = brush.add_vertices([
            local_x * size + local_z * size,
            - local_x * size + local_z * size,
            - local_x * size - local_z * size,
            local_x * size - local_z * size,
        ]);
        brush.add_polygon(&corners);
        brush
    }
}
