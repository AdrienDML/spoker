use crate::prelude::*;

use super::{ALine3d, APlane3d};

// A trait for computing the geometric intersection of 2 geomtric entities.
pub trait Intersection {
    type Lhs;
    type Rhs;

    fn intersect(lhs: &Self::Lhs, rhs: &Self::Rhs) -> Self;

    fn are_intersecting(lhs: &Self::Lhs, rhs: &Self::Rhs) -> bool;
}

// Intersection between a 3d point and the half space defined by a plane where the normal is
// poinging outwards.
pub enum PointHalfSpaceIntersection {
    Inside,
    OnBoundary,
    None,
}

impl Intersection for PointHalfSpaceIntersection {
    type Lhs = Vec3;
    type Rhs = APlane3d;

    fn are_intersecting(lhs: &Self::Lhs, rhs: &Self::Rhs) -> bool {
        let point = *lhs;
        let APlane3d { origin, normal: direction} = *rhs;
        let distance = (origin - point).dot(*direction);
        distance.abs() > f32::EPSILON         
    }

    fn intersect(lhs: &Self::Lhs, rhs: &Self::Rhs) -> Self {
        let point = *lhs;
        let APlane3d { origin, normal: direction} = *rhs;
        let distance = (origin - point).dot(*direction);
        if distance < 0.0 {
            Self::Inside
        } else if distance < f32::EPSILON {
            Self::OnBoundary
        } else {
            Self::None
        }
    }
}

// Intersection between a 3d line and a plane by a plane where the normal is
// poinging outwards.
pub enum ALinePlaneIntersection {
    Point(Vec3),
    OnBoundary,
    None,
}

impl Intersection for ALinePlaneIntersection {

    type Lhs = ALine3d;

    type Rhs = APlane3d;

    fn intersect(lhs: &Self::Lhs, rhs: &Self::Rhs) -> Self {
        let ALine3d {origin: l_origin, direction} = *lhs;
        let APlane3d {origin: p_origin, normal} = *rhs;
        let a = normal.dot(p_origin - l_origin);
        let b = normal.dot(*direction);
        if b.abs() < f32::EPSILON {
            if a.abs() < f32::EPSILON {
                ALinePlaneIntersection::OnBoundary
            } else {
                ALinePlaneIntersection::None
            }
        } else {
            Self::Point(l_origin + direction * a/b)
        }
    }

    fn are_intersecting(lhs: &Self::Lhs, rhs: &Self::Rhs) -> bool {
        let ALine3d {origin: l_origin, direction} = *lhs;
        let APlane3d {origin: p_origin, normal} = *rhs;
        let a = normal.dot(p_origin - l_origin);
        let b = normal.dot(*direction);
        b.abs() < f32::EPSILON && a.abs() > f32::EPSILON
    }
}

pub enum ALineHalfSpaceIntersection {
    Ray(Ray3d),
    OnBoundary,
    None,
}

impl Intersection for ALineHalfSpaceIntersection {

    type Lhs = ALine3d;

    type Rhs = APlane3d;

    fn intersect(lhs: &Self::Lhs, rhs: &Self::Rhs) -> Self {
        match ALinePlaneIntersection::intersect(lhs, rhs) {
            ALinePlaneIntersection::Point(p) => {
                let mut ray = Ray3d {origin: p, direction: lhs.direction};
                if ray.direction.dot(*rhs.normal) > 0.0 {
                    ray.direction = Direction3d::new(ray.direction.flip()).unwrap();
                }
                Self::Ray(ray)
            }
            ALinePlaneIntersection::None => Self::None,
            ALinePlaneIntersection::OnBoundary => Self::OnBoundary,
        }
    }

    fn are_intersecting(lhs: &Self::Lhs, rhs: &Self::Rhs) -> bool {
        let ALine3d {origin: l_origin, direction} = *lhs;
        let APlane3d {origin: p_origin, normal} = *rhs;
        let a = normal.dot(p_origin - l_origin);
        let b = normal.dot(*direction);
        b.abs() < f32::EPSILON && a.abs() > f32::EPSILON
    }
}
