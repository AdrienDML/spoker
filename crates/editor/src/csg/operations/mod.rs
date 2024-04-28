use crate::prelude::*;

use super::{BrushMesh, Plane};
use bevy::render::{
    mesh::Indices, render_asset::RenderAssetUsages, render_resource::PrimitiveTopology,
};

pub trait BrushMeshOperation {
    type Out;

    fn apply(self, brush: &BrushMesh) -> Self::Out;
}

pub enum Triangulate {
    EarClip,
    Delonay,
}

impl BrushMeshOperation for Triangulate {
    type Out = Mesh;

    fn apply(self, brush: &BrushMesh) -> Self::Out {
        match self {
            Self::EarClip => ear_clip(brush),
            Self::Delonay => delonay(brush),
        }
    }
}

fn ear_clip(brush: &BrushMesh) -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::all());
    let mut indicies = Vec::new();
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, brush.positions());

    for poly in brush.polygons() {
        let mut verts = poly.verticies().collect::<Vec<_>>();
        while verts.len() > 3 {
            let len = verts.len();
            'outer: for i in 0..len {
                let v1 = verts[i].data.point;
                let v2 = verts[(i + 1) % len].data.point;
                let v3 = verts[(i + 2) % len].data.point;
                // Check if its convex.
                let angle = (v1 - v2).angle_between(v3 - v2);
                if angle > std::f32::consts::PI
                {
                    continue;
                }
                // Check if no other point is in the triangle.
                for j in 0..len-3 {
                    let idx = (i + 3 + j) % len;
                    let point = verts[idx].point;
                    if point.in_triangle(v1, v2, v3) {
                        dbg!("Skip Ear");
                        continue 'outer;
                    }
                }
                indicies.extend(&[
                    verts[i].id as u32,
                    verts[(i + 1) % len].id as u32,
                    verts[(i + 2) % len].id as u32,
                ]);
                verts.remove((i + 1) % len);
                break;
            }
        }
        indicies.extend(&[
            verts[0].id as u32,
            verts[1].id as u32,
            verts[2].id as u32,
        ]);
    }
    mesh.insert_indices(Indices::U32(indicies));
    mesh.with_duplicated_vertices().with_computed_flat_normals()
}

fn delonay(_brush: &BrushMesh) -> Mesh {
    unimplemented!();
}

pub struct PlaneClip(Plane);

impl BrushMeshOperation for PlaneClip {
    type Out = BrushMesh;

    fn apply(self, _brush: &BrushMesh) -> Self::Out {
        todo!()
    }
}
