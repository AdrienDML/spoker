use crate::prelude::*;

pub mod intersection;

pub struct ALine3d {
    pub origin: Vec3,
    pub direction: Direction3d,
}

impl ALine3d {
    pub fn new(origin: Vec3, direction: Direction3d) -> Self {
        Self { origin, direction }
    }

    pub fn from_points(a: Vec3, b: Vec3) -> Self {
        Self::new(
            a,
            Direction3d::new(b - a).expect("ALine3d must be define by two non equal points."),
        )
    }

    pub fn dual(self) -> APlane3d {
        let ALine3d { origin, direction } = self;
        APlane3d { origin, normal: direction }
    }

    pub fn point_at(&self, t: f32) -> Vec3 {
        self.origin + t * *self.direction
    }

    pub fn segment(&self, end: f32) -> ASegment3d {
        ASegment3d::new(self.origin, self.point_at(end))
    }

    pub fn transform(&self, transform: Transform) -> Self {
        Self {
            origin: transform.transform_point(self.origin),
            direction: transform.rotation * self.direction,
        }
    }
}

pub struct ASegment3d {
    pub origin: Vec3,
    pub end: Vec3,
}

impl ASegment3d {
    pub fn new(origin: Vec3, end: Vec3) -> Self {
        Self { origin, end }
    }

    pub fn middle(&self) -> Vec3 {
        (self.origin+ self.end) * 0.5
    }

    pub fn lenght_squared(&self) -> f32 {
        (self.end - self.origin).length_squared()
    }

    pub fn lenght(&self) -> f32 {
        self.lenght_squared().sqrt()
    }

    pub fn direction(&self) -> Direction3d {
        Direction3d::new(self.end - self.origin).expect("The segment must be constitued from two different points.")
    }

    pub fn bisecting_plane(&self) -> APlane3d {
        APlane3d::new(self.middle(), self.direction())
    }

    pub fn transform(&self, tranform: &Transform) -> Self {
        Self {
            origin: tranform.transform_point(self.origin),
            end: tranform.transform_point(self.end),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct APlane3d {
    pub origin: Vec3,
    pub normal: Direction3d,
}

impl APlane3d {
    pub fn new(origin: Vec3, direction: Direction3d) -> Self {
        Self { origin, normal: direction }
    }

    pub fn from_points(a: Vec3, b: Vec3, c: Vec3) -> Self {
        Self::new(
            a,
            Direction3d::new((b - a).cross(c - a)).expect(
                "APlane3d must be defined by three finite points that don't lie on the same line.",
            ),
        )
    }

    pub fn dual(self) -> APlane3d {
        let APlane3d { origin, normal: direction } = self;
        Self { origin, normal: direction }
    }

    pub fn transform(&self, transform: Transform) -> Self {
        Self {
            origin: transform.transform_point(self.origin),
            normal: transform.rotation * self.normal,
        }
    }
}
