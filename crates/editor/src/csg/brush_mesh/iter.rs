use super::*;

pub struct BrushVerticies<'b> {
    pub brush: &'b BrushMesh,
    pub current_vertice: VerticeId,
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
    pub current_edge: HalfEdgeId,
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
    pub current_polygon: PolygonId,
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
    pub current_vertice: usize,
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
    pub current_edge: usize,
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
