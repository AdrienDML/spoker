use super::*;

impl BrushMesh {
    pub fn polygons_mut(&mut self) -> BrushPolygonsMut {
        BrushPolygonsMut {
            brush: self,
            current_polygon: 0,
        }
    }

    pub fn vertices_mut(&mut self) -> BrushVerticiesMut {
        BrushVerticiesMut {
            brush: self,
            current_vertice: 0,
        }
    }

    pub fn edges_mut(&mut self) -> BrushEdgesMut {
        BrushEdgesMut {
            brush: self,
            current_edge: 0,
        }
    }
}

pub struct BrushVerticiesMut<'b> {
    pub brush: &'b mut BrushMesh,
    pub(super) current_vertice: VerticeId,
}

#[derive(Deref)]
pub struct BrushVerticeMut<'b> {
    pub brush: &'b mut BrushMesh,
    #[deref]
    pub data: &'b mut Vertice,
    pub id: VerticeId,
}

impl<'b> Iterator for BrushVerticiesMut<'b> {
    type Item = BrushVertice<'b>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_vertice < self.brush.verticies.len() {
            let vert = BrushVertice {
                brush: self.brush,
                data: &mut self.brush.verticies[self.current_vertice],
                id: self.current_vertice,
            };
            self.current_vertice += 1;
            Some(vert)
        } else {
            None
        }
    }
}

pub struct BrushEdgesMut <'b> {
    pub brush: &'b mut BrushMesh,
    pub(super) current_edge: HalfEdgeId,
}

#[derive(Deref)]
pub struct BrushEdge<'b> {
    pub brush: &'b mut BrushMesh,
    #[deref]
    pub data: &'b mut HalfEdge,
    pub id: HalfEdgeId,
}

impl<'b> Iterator for BrushEdgesMut <'b> {
    type Item = BrushEdge<'b>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_edge < self.brush.half_edges.len() {
            let edge = BrushEdge {
                brush: self.brush,
                data: &mut self.brush.half_edges[self.current_edge],
                id: self.current_edge,
            };
            self.current_edge += 1;
            Some(edge)
        } else {
            None
        }
    }
}

pub struct BrushPolygonsMut<'b> {
    pub brush: &'b mut BrushMesh,
    pub(super) current_polygon: PolygonId,
}

#[derive(Deref)]
pub struct BrushPolygonMut<'b> {
    pub brush: &'b mut BrushMesh,
    #[deref]
    pub data: &'b mut Polygon,
    pub id: PolygonId,
}

impl<'b> BrushPolygonMut <'b> {
    pub fn verticies(&'b self) -> PolygonVertices<'b> {
        PolygonVertices {
            poly: &BrushPolygon {
                brush: self.brush,
                data: self.data,
                id: self.id,
            },
            current_vertice: 0usize,
        }
    }

    pub fn verticies_mut(&'b mut self) -> PolygonVerticesMut <'b> {
        PolygonVerticesMut {
            poly: self,
            current_vertice: 0usize,
        }
    }
}

impl<'b> Iterator for BrushPolygonsMut <'b> {
    type Item = BrushPolygonMut<'b>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_polygon < self.brush.polygons.len() {
            let poly = BrushPolygonMut {
                brush: self.brush,
                data: &mut self.brush.polygons[self.current_polygon],
                id: self.current_polygon,
            };
            self.current_polygon += 1;
            Some(poly)
        } else {
            None
        }
    }
}

pub struct PolygonVerticesMut<'b> {
    pub poly: &'b mut BrushPolygonMut<'b>,
    pub(super) current_vertice: usize,
}

#[derive(Deref)]
pub struct PolygonVerticeMut <'b> {
    pub poly: &'b mut BrushPolygonMut<'b>,
    #[deref]
    pub data: &'b mut Vertice,
    pub id: VerticeId,
}

impl<'b> Iterator for PolygonVerticesMut<'b> {
    type Item = PolygonVerticeMut<'b>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_vertice < self.poly.data.verticies.len() {
            let vert = PolygonVerticeMut {
                poly: self.poly,
                data: self
                    .poly
                    .brush
                    .get_vertice_mut(self.poly.data.verticies[self.current_vertice]),
                id: self.poly.data.verticies[self.current_vertice],
            };
            self.current_vertice += 1;
            Some(vert)
        } else {
            None
        }
    }
}

pub struct PolygonEdgesMut<'b> {
    pub poly: &'b mut BrushPolygonMut<'b>,
    pub(super) current_edge: usize,
}

#[derive(Deref)]
pub struct PolygonEdgeMut<'b> {
    pub poly: &'b mut BrushPolygonMut<'b>,
    #[deref]
    pub data: &'b mut HalfEdge,
    pub id: HalfEdgeId,
}

impl<'b> Iterator for PolygonEdgesMut<'b> {
    type Item = PolygonEdgeMut<'b>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_edge < self.poly.data.half_edges.len() {
            let edge = PolygonEdgeMut {
                poly: self.poly,
                data: self
                    .poly
                    .brush
                    .get_half_edge_mut(self.poly.data.half_edges[self.current_edge]),
                id: self.poly.data.half_edges[self.current_edge],
            };
            self.current_edge += 1;
            Some(edge)
        } else {
            None
        }
    }
}

pub struct BrushPlanesMut<'b> {
    pub brush: &'b mut BrushMesh,
    pub(super) current_plane: PlaneId,
}

#[derive(Deref)]
pub struct BrushPlaneMut<'b> {
    pub brush: &'b mut BrushMesh,
    #[deref]
    pub data: &'b mut Plane,
    pub id: PlaneId,
}

impl<'b> Iterator for BrushPlanesMut<'b> {
    type Item = BrushPlaneMut<'b>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_plane < self.brush.planes.len() {
            let plane = BrushPlaneMut {
                brush: self.brush,
                data: self.brush.get_plane_mut(self.current_plane),
                id: self.current_plane,
            };
            self.current_plane += 1;
            Some(plane)
        } else {
            None
        }
    }
}

pub struct PlanePolygonsMut<'b> {
    pub plane: &'b mut BrushPlaneMut<'b>,
    pub(super) current_polygon: usize,
}

#[derive(Deref)]
pub struct PlanePolygonMut<'b> {
    pub plane: &'b mut BrushPlaneMut<'b>,
    #[deref]
    pub data: &'b Polygon,
    pub id: PolygonId,
}

impl<'b> Iterator for PlanePolygonsMut<'b> {
    type Item = PlanePolygonMut<'b>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_polygon < self.plane.data.polygons.len() {
            let plane = PlanePolygonMut {
                plane: self.plane,
                data: self
                    .plane
                    .brush
                    .get_polygon(self.plane.data.polygons[self.current_polygon]),
                id: self.plane.data.polygons[self.current_polygon],
            };
            self.current_polygon += 1;
            Some(plane)
        } else {
            None
        }
    }
}

pub struct PlaneHolesMut<'b> {
    pub plane: &'b mut BrushPlaneMut<'b>,
    pub(super) current_hole: usize,
}

#[derive(Deref)]
pub struct PlaneHoleMut<'b> {
    pub plane: &'b mut BrushPlaneMut<'b>,
    #[deref]
    pub data: &'b mut Polygon,
    pub id: PolygonId,
}

impl<'b> Iterator for PlaneHolesMut<'b> {
    type Item = PlaneHoleMut<'b>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_hole < self.plane.data.holes.len() {
            let id = self.plane.data.holes[self.current_hole];
            let plane = PlaneHoleMut {
                plane: self.plane,
                data: self
                    .plane
                    .brush
                    .get_polygon_mut(id),
                id,
            };
            self.current_hole += 1;
            Some(plane)
        } else {
            None
        }
    }
}
