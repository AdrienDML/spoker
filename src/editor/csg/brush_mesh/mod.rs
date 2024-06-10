use std::array;

mod iter;
//mod iter_mut;

use bevy::math::bounding::Bounded3d;
pub use iter::*;

use crate::prelude::*;

pub type VerticeId = usize;

#[derive(Reflect, Clone)]
pub struct Vertice {
    pub point: Vec3,
    out_half_edges: Vec<HalfEdgeId>,
    in_half_edges: Vec<HalfEdgeId>,
}

impl Vertice {
    pub fn from_point(point: Vec3) -> Self {
        Self {
            point,
            out_half_edges: Vec::new(),
            in_half_edges: Vec::new(),
        }
    }

    pub fn add_out_edge(&mut self, id: HalfEdgeId) {
        self.out_half_edges.push(id);
    }

    pub fn add_in_edge(&mut self, id: HalfEdgeId) {
        self.in_half_edges.push(id);
    }
}

pub type HalfEdgeId = usize;

// A half edges is a directed edge from a vertex to another.
#[derive(Reflect, PartialEq, Eq, Hash, Clone)]
pub struct HalfEdge {
    pub origin: VerticeId,
    pub end: VerticeId,
    pub polygon: PolygonId,
}

impl HalfEdge {
    fn direction(&self, vertices: &[Vertice]) -> Vec3 {
        vertices[self.end].point - vertices[self.origin].point
    }

    fn normalized_direction(&self, vertices: &[Vertice]) -> Vec3 {
        self.direction(vertices).normalize()
    }
}

pub type PlaneId = usize;

#[derive(Reflect, Clone)]
pub struct Plane {
    pub point: Vec3,
    pub normal: Vec3,
    polygons: Vec<PolygonId>,
    holes: Vec<PolygonId>,
}

impl Plane {
    pub fn new(point: Vec3, normal: Vec3) -> Self {
        Self {
            point,
            normal,
            polygons: Vec::new(),
            holes: Vec::new(),
        }
    }

    fn add_polygon(&mut self, polygon: PolygonId) {
        self.polygons.push(polygon);
    }

    fn add_hole(&mut self, hole: PolygonId) {
        self.holes.push(hole);
    }

}

pub type PolygonId = usize;

#[derive(Reflect, Clone)]
pub struct Polygon {
    pub verticies: Vec<VerticeId>,
    pub half_edges: Vec<HalfEdgeId>,
    pub plane: PlaneId,
}

impl Polygon {
    fn is_on_plane(&self, edges: &[HalfEdge], planes: &[Plane], vertices: &[Vertice]) -> bool {
        let normal = planes[self.plane].normal;
        for edge in &self.half_edges {
            let edge = &edges[*edge];
            let dir = edge.direction(vertices);
            if dir.cross(normal).length_squared() < f32::EPSILON {
                continue;
            } else {
                return false;
            }
        }
        true
    }

    fn is_convex(&self, edges: &[HalfEdge], vertices: &[Vertice]) -> bool {
        let nb_edges = self.half_edges.len();
        for edge in 0..nb_edges {
            let cur = edges[edge].direction(vertices).flip();
            let next = edges[(edge + 1) % nb_edges].direction(vertices);
            if (next.angle_between(cur).abs() - 180.0).abs() > f32::EPSILON {
                return false;
            }
        }
        true
    }
}

#[derive(Asset, Reflect, Clone)]
pub struct BrushMesh {
    verticies: Vec<Vertice>,
    half_edges: Vec<HalfEdge>,
    polygons: Vec<Polygon>,
    planes: Vec<Plane>,
}

impl BrushMesh {
    pub fn empty() -> Self {
        Self {
            verticies: Vec::new(),
            half_edges: Vec::new(),
            polygons: Vec::new(),
            planes: Vec::new(),
        }
    }

    pub fn add_vertice(&mut self, point: Vec3) -> VerticeId {
        self.verticies.push(Vertice::from_point(point));
        self.verticies.len() - 1
    }

    pub fn add_vertices<const N: usize>(&mut self, points: [Vec3; N]) -> [VerticeId; N] {
        let start = self.verticies.len();
        self.verticies
            .extend(points.into_iter().map(Vertice::from_point));
        array::from_fn(|i| start + i)
    }

    pub fn extend_verticies(&mut self, points: impl IntoIterator<Item = Vec3>) -> Vec<VerticeId> {
        let start = self.verticies.len();
        self.verticies
            .extend(points.into_iter().map(Vertice::from_point));
        (start..(self.verticies.len() - 1)).collect()
    }

    pub fn add_plane(&mut self, normal: Vec3, point: Vec3) -> PlaneId {
        self.planes.push(Plane::new(normal, point));
        self.planes.len()
    }

    pub fn add_planes<const N: usize>(&mut self, planes: [Plane; N]) -> [PlaneId; N] {
        let start = self.planes.len();
        self.planes.extend(planes);
        array::from_fn(|i| start + i)
    }

    pub fn extend_planes(&mut self, planes: impl IntoIterator<Item = Plane>) -> Vec<VerticeId> {
        let start = self.planes.len();
        self.planes.extend(planes.into_iter());
        (start..(self.verticies.len() - 1)).collect()
    }

    pub fn get_vertice(&self, id: VerticeId) -> &Vertice {
        &self.verticies[id]
    }

    pub fn get_vertice_mut(&mut self, id: VerticeId) -> &mut Vertice {
        &mut self.verticies[id]
    }

    pub fn get_half_edge(&self, id: HalfEdgeId) -> &HalfEdge {
        &self.half_edges[id]
    }

    pub fn get_half_edge_mut(&mut self, id: HalfEdgeId) -> &mut HalfEdge {
        &mut self.half_edges[id]
    }

    pub fn get_plane(&self, id: PlaneId) -> &Plane {
        &self.planes[id]
    }

    pub fn get_polygon(&self, id: PolygonId) -> &Polygon {
        &self.polygons[id]
    }

    pub fn add_polygon_on_plane(&mut self, vertices: &[VerticeId], plane: PlaneId) -> PolygonId {
        assert!(
            vertices.len() >= 3,
            "Tried to construct a polygon of length less than 3."
        );
        let polygon_id = self.polygons.len();
        let mut half_edges = Vec::with_capacity(vertices.len());
        let prev_vertice_id = vertices[0];
        vertices
            .iter()
            .cycle()
            .skip(1)
            .take(vertices.len() - 1)
            .for_each(|vertice_id| {
                let half_edge_id = self.half_edges.len();
                half_edges.push(half_edge_id);
                let prev_vertice = self.get_vertice_mut(prev_vertice_id);
                prev_vertice.out_half_edges.push(half_edge_id);
                let vertice = self.get_vertice_mut(*vertice_id);
                vertice.in_half_edges.push(half_edge_id);
                self.half_edges.push(HalfEdge {
                    origin: prev_vertice_id,
                    end: *vertice_id,
                    polygon: polygon_id,
                });
            });

        self.polygons.push(Polygon {
            half_edges,
            verticies: vertices.into(),
            plane,
        });
        polygon_id
    }

    /// Add a polygon constructing the necessary half edges.
    /// It also adds the contructed half edges and the plane the vertices lie on.
    pub fn add_polygon(&mut self, vertices: &[VerticeId]) -> PolygonId {
        let scaling_factor = vertices.len() as f32;
        assert!(
            scaling_factor >= 3.0,
            "Tried to construct a polygon of length less than 3."
        );
        let polygon_id = self.polygons.len();
        let mut half_edges = Vec::with_capacity(vertices.len());

        // Construct the half edges if they don't exist.
        let mut plane_center = Vec3::ZERO;
        let prev_vertice_id = vertices[0];
        vertices
            .iter()
            .cycle()
            .skip(1)
            .take(vertices.len() - 1)
            .for_each(|vertice_id| {
                let half_edge_id = self.half_edges.len();
                half_edges.push(half_edge_id);
                let prev_vertice = self.get_vertice_mut(prev_vertice_id);
                prev_vertice.out_half_edges.push(half_edge_id);
                plane_center += prev_vertice.point * scaling_factor;
                let vertice = self.get_vertice_mut(*vertice_id);
                vertice.in_half_edges.push(half_edge_id);

                self.half_edges.push(HalfEdge {
                    origin: prev_vertice_id,
                    end: *vertice_id,
                    polygon: polygon_id,
                });
            });

        // Compute plane Normal.
        let plane_normal = {
            let a = self.get_vertice(vertices[0]).point;
            let b = self.get_vertice(vertices[1]).point;
            let c = self.get_vertice(vertices[2]).point;
            (b - a).cross(c - a)
        };
        let plane = self.add_plane(plane_normal, plane_center);

        self.polygons.push(Polygon {
            half_edges,
            verticies: vertices.into(),
            plane,
        });
        polygon_id
    }


    pub fn positions(&self) -> Vec<Vec3> {
        self.verticies.iter().map(|v| v.point).collect()
    }

    fn get_plane_mut(&mut self, id: PlaneId) -> &mut Plane {
        &mut self.planes[id]
    }

    fn get_polygon_mut(&mut self, id: PlaneId) -> &mut Polygon {
        &mut self.polygons[id]
    }
}

impl Bounded3d for BrushMesh {
    fn aabb_3d(&self, translation: Vec3, rotation: Quat) -> bevy::math::bounding::Aabb3d {
        let (min, max) =
            self.verticies
                .iter()
                .fold((Vec3::NEG_INFINITY, Vec3::INFINITY), |state, next| {
                    let next = rotation * next.point + translation;
                    (state.0.min(next), state.1.max(next))
                });
        bevy::math::bounding::Aabb3d { min, max }
    }

    fn bounding_sphere(
        &self,
        translation: Vec3,
        rotation: Quat,
    ) -> bevy::math::bounding::BoundingSphere {
        let aabb = self.aabb_3d(translation, rotation);
        let center = (aabb.min + aabb.max) * 0.5;
        let radius = self
            .verticies
            .iter()
            .fold(0.0, |r: f32, v| r.max(v.point.length()));
        bevy::math::bounding::BoundingSphere {
            center,
            sphere: Sphere { radius },
        }
    }
}
