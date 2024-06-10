use common::geometry::APlane3d;


use crate::csg::BrushMesh;

use super::BrushMeshOperation;

pub struct PlaneClip(APlane3d);

impl BrushMeshOperation for PlaneClip {
    type Out = BrushMesh;

    fn apply(self, _brush: &BrushMesh) -> Self::Out {
        let result = BrushMesh::empty();
        let PlaneClip(plane) = self;
        for poly in _brush.polygons() {
            let plane = _brush.get_plane(poly.plane);
            // If the planes are colinear there is no clipping.
            if (plane.normal.cross(normal).length_squared() < f32::EPSILON) && point != plane.point
            {
                if plane.point.intersect_plane(normal ,point) {

                }
                continue;
            }

            for _edge in poly.edges() {
            }
        }
        unimplemented!()
    }
}

