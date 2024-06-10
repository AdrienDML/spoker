use super::BrushMesh;

//mod clip;
//pub use clip::*;
mod triangulate;
pub use triangulate::*;

/// A trait for applying operation to a BrushMesh producing a new Brush.
pub trait BrushMeshOperation {
    type Out;

    fn apply(self, brush: &BrushMesh) -> Self::Out;
}
