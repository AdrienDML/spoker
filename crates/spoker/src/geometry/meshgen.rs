use bevy::prelude::*;

use crate::geometry::{
    intersections::{Intersection, RayPlaneIntersection, LinePlaneIntersection},
    shapes::{Line, Ray},
};

use super::{intersections::PointPlaneIntersection, shapes::Plane};

// A polygon defined by its vertices in clock wise order.
pub struct Poly {
    vertices: Vec<Vec3>,
    normal: Vec3,
}

pub struct Triangulation {
    vertices: Vec<Vec3>,
    indices: Vec<[usize; 3]>,
}

impl Poly {
    // Triangulation using ear clipping method.
    fn triangulate(mut self) -> Triangulation {
        let mut indices = vec![];
        let mut vertices = vec![];
        while self.vertices.len() > 3 {
            let mut idx = 0;
            while idx < self.vertices.len() {
                let (a, b, c) = (
                    self.vertices[idx],
                    self.vertices[(idx + 1) % self.vertices.len()],
                    self.vertices[(idx + 2) % self.vertices.len()],
                );
                let val = (b - a).cross(c - a).dot(self.normal);
                if val > f32::EPSILON {
                    let nb = vertices.len();
                    vertices.extend([a, b, c]);
                    indices.push([nb, nb + 1, nb + 2]);
                    self.vertices.remove(idx + 1);
                } else {
                    continue;
                }
                idx += 1;
            }
        }
        let nb = vertices.len();
        vertices.extend(self.vertices);
        indices.push([nb, nb + 1, nb + 2]);
        Triangulation { vertices, indices }
    }

    fn plane_clip(&mut self, plane: Plane) {
        use PointPlaneIntersection as PPI;
        use LinePlaneIntersection as LPI;
        // If plane doesn't intersect return.
        if self.normal.cross(plane.normal).length_squared() < f32::EPSILON {
            return;
        }
        let mut vertices = vec![];
        let pvert = self.vertices[0];
        for &vert in self.vertices.iter().skip(1) {
            match (
                PPI::intersection(pvert, plane),
                PPI::intersection(vert, plane),
            ) {
                (PPI::Outside, PPI::Outside) => {
                    vertices.push(pvert);
                }
                (PPI::Outside, PPI::Inside) => {
                    let LPI::FromOutside { point } =
                        LPI::intersection(Line::from_points(pvert, vert), plane)
                    else {
                        panic!("something went wrong in plane clipping.")
                    };
                    vertices.push(point);
                }
                (PPI::Inside, PPI::Outside) => {
                    let LPI::FromInside { point } =
                        LPI::intersection(Line::from_points(pvert, vert), plane)
                    else {
                        panic!("something went wrong in plane clipping.")
                    };
                    vertices.push(point);
                }
                (_, _) => continue,
            }
        }
        self.vertices = vertices;
    }

    fn from_plane_in_box(plane: Plane, size: f32) -> Self {
        let edges = [
            Ray::new(Vec3::splat(size), Vec3::NEG_X, size),
            Ray::new(Vec3::splat(size), Vec3::NEG_Y, size),
            Ray::new(Vec3::splat(size), Vec3::NEG_Z, size),
            Ray::new(Vec3::splat(-size), Vec3::X, size),
            Ray::new(Vec3::splat(-size), Vec3::Y, size),
            Ray::new(Vec3::splat(-size), Vec3::Z, size),
            Ray::new(Vec3::new(size, -size, size), Vec3::NEG_X, size),
            Ray::new(Vec3::new(size, -size, size), Vec3::NEG_Z, size),
            Ray::new(Vec3::new(-size, size, -size), Vec3::X, size),
            Ray::new(Vec3::new(-size, size, -size), Vec3::Z, size),
            Ray::new(Vec3::new(size, size, -size), Vec3::Y, size),
            Ray::new(Vec3::new(-size, size, size), Vec3::Y, size),
        ];
        let points = vec![];
        for edge in edges {
            match RayPlaneIntersection::intersection(edge, plane) {
                RayPlaneIntersection::FromInside { point } |
                RayPlaneIntersection::FromOutside { point } => {
                    points.push(point)
                }
                _ => continue,
            }
        }
        points[1..].sort_unstable_by(|a, b| if (points[0] - plane.origin).cross(b - plane.origin) )
    }
}
