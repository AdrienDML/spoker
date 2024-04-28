use std::array;

mod iter;
pub use iter::*;

use bevy::{
    math::bounding::Bounded3d,
    prelude::*,
};

pub type VerticeId = usize;

#[derive(Reflect)]
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
#[derive(Reflect, PartialEq, Eq, Hash)]
pub struct HalfEdge {
    pub origin: VerticeId,
    pub end: VerticeId,
    pub polygon: PolygonId,
}

pub type PolygonId = usize;

#[derive(Reflect)]
pub struct Plane {
    point: Vec3,
    normal: Vec3,
}

// Polygon constructed from directed half edges. 
#[derive(Reflect)]
pub struct Polygon {
    pub verticies: Vec<VerticeId>,
    pub half_edges: Vec<HalfEdgeId>,
    pub plane: Plane,
}

#[derive(Asset, Reflect, Component)]
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

    pub fn add_polygons<const N: usize>(&mut self, polygons: [&[VerticeId]; N]) -> [PolygonId; N] {
        let start = self.polygons.len();
        polygons
            .into_iter()
            .for_each(|polygon| { self.add_polygon(polygon); });
        array::from_fn(|i| start + i)
    }

    pub fn extend_polygons<'a, const N: usize>(
        &mut self,
        polygons: impl IntoIterator<Item = &'a [VerticeId]>,
    ) -> [PolygonId; N] {
        let start = self.polygons.len();
        polygons
            .into_iter()
            .for_each(|polygon| { self.add_polygon(polygon); });
        array::from_fn(|i| start + i)
    }

    /// Add a polygon constructing the necessary half edges.
    /// It also adds the contructed half edges
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

        self.polygons.push(Polygon {
            half_edges,
            verticies: vertices.into(),
            plane: Plane {
                point: plane_center,
                normal: plane_normal,
            },
        });
        polygon_id
    }

    pub fn polygons(&self) -> BrushPolygons {
        BrushPolygons {
            brush: self,
            current_polygon: 0,
        }
    }

    pub fn vertices(&self) -> BrushVerticies {
        BrushVerticies {
            brush: self,
            current_vertice: 0,
        }
    }

    pub fn edges(&self) -> BrushEdges {
        BrushEdges {
            brush: self,
            current_edge: 0,
        }
    }

    pub fn positions(&self) -> Vec<Vec3> {
        self.verticies.iter().map(|v| v.point).collect()
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
