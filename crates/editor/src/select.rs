use crate::{csg::brush_mesh::{HalfEdgeId, PolygonId, VerticeId}, prelude::*};

pub struct SelectPlugin; 

impl Plugin for SelectPlugin {
    fn build(&self, app: &mut App) {
        todo!()
    }
}

#[derive(Resource)]
pub enum SelectMode {
    Object,
    Vertex,
    Edge,
    Face,
}

impl SelectMode {
    fn next(&self) -> Self {
        match self {
            Self::Object => Self::Vertex,
            Self::Vertex => Self::Edge,
            Self::Edge => Self::Face,
            Self::Face => Self::Object,
        }
    }
}

pub fn cycle_selectmode(
    input: Res<ButtonInput<KeyCode>>,
    mut mode: ResMut<SelectMode>,
) {
    if input.just_pressed(KeyCode::Tab) {
        *mode = mode.next()
    }
}

#[derive(Component)]
pub struct Selection {
    verticies: Vec<VerticeId>,
    edges: Vec<HalfEdgeId>,
    polygons: Vec<PolygonId>,
}
