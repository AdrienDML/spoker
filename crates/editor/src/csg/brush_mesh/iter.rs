use super::*;

impl BrushMesh {
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
}

pub struct BrushVerticies<'b> {
    pub brush: &'b BrushMesh,
    pub(super) current_vertice: VerticeId,
}

#[derive(Deref)]
pub struct BrushVertice<'b> {
    pub brush: &'b BrushMesh,
    #[deref]
    pub data: &'b Vertice,
    pub id: VerticeId,
}

impl<'b> Iterator for BrushVerticies<'b> {
    type Item = BrushVertice<'b>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_vertice < self.brush.verticies.len() {
            let vert = BrushVertice {
                brush: self.brush,
                data: &self.brush.verticies[self.current_vertice],
                id: self.current_vertice,
            };
            self.current_vertice += 1;
            Some(vert)
        } else {
            None
        }
    }
}

pub struct BrushEdges<'b> {
    pub brush: &'b BrushMesh,
    pub(super) current_edge: HalfEdgeId,
}

#[derive(Deref)]
pub struct BrushEdge<'b> {
    pub brush: &'b BrushMesh,
    #[deref]
    pub data: &'b HalfEdge,
    pub id: HalfEdgeId,
}

impl<'b> Iterator for BrushEdges<'b> {
    type Item = BrushEdge<'b>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_edge < self.brush.half_edges.len() {
            let edge = BrushEdge {
                brush: self.brush,
                data: &self.brush.half_edges[self.current_edge],
                id: self.current_edge,
            };
            self.current_edge += 1;
            Some(edge)
        } else {
            None
        }
    }
}

pub struct BrushPolygons<'b> {
    pub brush: &'b BrushMesh,
    pub(super) current_polygon: PolygonId,
}

#[derive(Deref)]
pub struct BrushPolygon<'b> {
    pub brush: &'b BrushMesh,
    #[deref]
    pub data: &'b Polygon,
    pub id: PolygonId,
}

impl<'b> BrushPolygon<'b> {
    pub fn verticies(&'b self) -> PolygonVertices<'b> {
        PolygonVertices {
            poly: self,
            current_vertice: 0usize,
        }
    }

    pub fn edges(&'b self) -> PolygonEdges<'b> {
        PolygonEdges {
            poly: self,
            current_edge: 0usize,
        }
    }
}

impl<'b> Iterator for BrushPolygons<'b> {
    type Item = BrushPolygon<'b>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_polygon < self.brush.polygons.len() {
            let poly = BrushPolygon {
                brush: self.brush,
                data: &self.brush.polygons[self.current_polygon],
                id: self.current_polygon,
            };
            self.current_polygon += 1;
            Some(poly)
        } else {
            None
        }
    }
}

pub struct PolygonVertices<'b> {
    pub poly: &'b BrushPolygon<'b>,
    current_vertice: usize,
}

#[derive(Deref)]
pub struct PolygonVertice<'b> {
    pub poly: &'b BrushPolygon<'b>,
    #[deref]
    pub data: &'b Vertice,
    pub id: VerticeId,
}

impl<'b> Iterator for PolygonVertices<'b> {
    type Item = PolygonVertice<'b>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_vertice < self.poly.data.verticies.len() {
            let vert = PolygonVertice {
                poly: self.poly,
                data: self
                    .poly
                    .brush
                    .get_vertice(self.poly.data.verticies[self.current_vertice]),
                id: self.poly.data.verticies[self.current_vertice],
            };
            self.current_vertice += 1;
            Some(vert)
        } else {
            None
        }
    }
}

pub struct PolygonEdges<'b> {
    pub poly: &'b BrushPolygon<'b>,
    current_edge: usize,
}

#[derive(Deref)]
pub struct PolygonEdge<'b> {
    pub poly: &'b BrushPolygon<'b>,
    #[deref]
    pub data: &'b HalfEdge,
    pub id: HalfEdgeId,
}

impl<'b> Iterator for PolygonEdges<'b> {
    type Item = PolygonEdge<'b>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_edge < self.poly.data.half_edges.len() {
            let edge = PolygonEdge {
                poly: self.poly,
                data: self
                    .poly
                    .brush
                    .get_half_edge(self.poly.data.half_edges[self.current_edge]),
                id: self.poly.data.half_edges[self.current_edge],
            };
            self.current_edge += 1;
            Some(edge)
        } else {
            None
        }
    }
}

pub struct BrushPlanes<'b> {
    pub brush: &'b BrushMesh,
    current_plane: PlaneId,
}

#[derive(Deref)]
pub struct BrushPlane<'b> {
    pub brush: &'b BrushMesh,
    #[deref]
    pub data: &'b Plane,
    pub id: PlaneId,
}

impl<'b> Iterator for BrushPlanes<'b> {
    type Item = BrushPlane<'b>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_plane < self.brush.planes.len() {
            let plane = BrushPlane {
                brush: self.brush,
                data: self.brush.get_plane(self.current_plane),
                id: self.current_plane,
            };
            self.current_plane += 1;
            Some(plane)
        } else {
            None
        }
    }
}

pub struct PlanePolygons<'b> {
    pub plane: &'b BrushPlane<'b>,
    current_polygon: usize,
}

#[derive(Deref)]
pub struct PlanePolygon<'b> {
    pub plane: &'b BrushPlane<'b>,
    #[deref]
    pub data: &'b Polygon,
    pub id: PolygonId,
}

impl<'b> Iterator for PlanePolygons<'b> {
    type Item = PlanePolygon<'b>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_polygon < self.plane.data.polygons.len() {
            let plane = PlanePolygon {
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

pub struct PlaneHoles<'b> {
    pub plane: &'b BrushPlane<'b>,
    current_hole: usize,
}

#[derive(Deref)]
pub struct PlaneHole<'b> {
    pub plane: &'b BrushPlane<'b>,
    #[deref]
    pub data: &'b Polygon,
    pub id: PolygonId,
}

impl<'b> Iterator for PlaneHoles<'b> {
    type Item = PlaneHole<'b>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_hole < self.plane.data.holes.len() {
            let id = self.plane.data.holes[self.current_hole];
            let plane = PlaneHole {
                plane: self.plane,
                data: self
                    .plane
                    .brush
                    .get_polygon(id),
                id,
            };
            self.current_hole += 1;
            Some(plane)
        } else {
            None
        }
    }
}
