use crate::physics::{
    bsp::{BspNode, MapBsp},
    physmat::PhysMat,
};

use bevy::{prelude::*, math::vec2};

pub trait Intersection<'l, 'r> {
    type Lhs: 'l;
    type Rhs: 'r;
    fn intersection(lhs: Self::Lhs, rhs: Self::Rhs) -> Self;
}

pub enum PointPlaneIntersection {
    Inside,
    Outside,
    OnBoundary,
}

impl<'l, 'r> Intersection<'l, 'r> for PointPlaneIntersection {
    type Lhs = Vec3;

    type Rhs = Plane3d;

    fn intersection(lhs: Self::Lhs, rhs: Self::Rhs) -> Self {
        let point = lhs;
        let plane = rhs;
        let dot = plane.signed_distance(point);
        if dot.abs() <= f32::EPSILON {
            Self::OnBoundary
        } else if dot > 0.0 {
            Self::Outside
        } else {
            Self::Inside
        }
    }
}

pub enum PointCylinderIntersection {
    Inside,
    OnBoundary,
    None,
}

impl<'l, 'r> Intersection<'l, 'r> for PointCylinderIntersection {
    type Lhs = Vec3;

    type Rhs = Capsule;

    fn intersection(lhs: Self::Lhs, rhs: Self::Rhs) -> Self {
        let mut point = lhs;
        let capsule = rhs;
        let ds = vec2(point.xz().length() - capsule.radius, point.y.abs() - capsule.half_height);
        let d = ds.x.max(ds.y).min(0.0) + ds.max(Vec2::splat(0.0)).length();
        if d <= f32::EPSILON {
            Self::OnBoundary
        } else if d < 0.0 {
            Self::Inside
        } else {
            Self::None
        }
    }
}

pub enum PointCapsuleIntersection {
    Inside,
    OnBoundary,
    None,
}

impl<'l, 'r> Intersection<'l, 'r> for PointCapsuleIntersection {
    type Lhs = Vec3;

    type Rhs = Capsule;

    fn intersection(lhs: Self::Lhs, rhs: Self::Rhs) -> Self {
        let mut point = lhs;
        let capsule = rhs;
        let d = capsule.signed_distance(point);
        if d <= f32::EPSILON {
            Self::OnBoundary
        } else if d < 0.0 {
            Self::Inside
        } else {
            Self::None
        }
    }
}

pub enum LinePlaneIntersection {
    // The point of intersection. If the ray origin is on the outside of the plane.
    FromInside { point: Vec3 },

    // The point of intersection. If the ray origin is on the inside of the plane.
    FromOutside { point: Vec3 },

    // The ray origin is on the plane boundary.
    OnBoundary,

    // No interseciton.
    None,
}

impl<'l, 'r> Intersection<'l, 'r> for LinePlaneIntersection {
    type Lhs = Line;

    type Rhs = Plane;

    fn intersection(lhs: Self::Lhs, rhs: Self::Rhs) -> Self {
        let line = lhs;
        let plane = rhs;

        let t = plane.signed_distance_on_dir(line);
        let is_intersecting = line.dir.dot(plane.normal) > 0.0;
        match PointPlaneIntersection::intersection(line.origin, plane) {
            PointPlaneIntersection::Inside => {
                if is_intersecting {
                    Self::FromInside { point: line.point_at(t) }
                } else {
                    Self::None
                }
            }
            PointPlaneIntersection::Outside => {
                if is_intersecting {
                    Self::FromOutside { point: line.point_at(t) }
                } else {
                    Self::None
                }
            }
            PointPlaneIntersection::OnBoundary => Self::OnBoundary,
        }
    }
}

pub enum RayPlaneIntersection {
    // The point of intersection. If the ray origin is on the outside of the plane.
    FromInside { point: Vec3 },

    // The point of intersection. If the ray origin is on the inside of the plane.
    FromOutside { point: Vec3 },

    // The ray origin is on the plane boundary.
    OnBoundary,

    // No interseciton.
    None,
}

impl<'l, 'r> Intersection<'l, 'r> for RayPlaneIntersection {
    type Lhs = Ray;

    type Rhs = Plane;

    fn intersection(lhs: Self::Lhs, rhs: Self::Rhs) -> Self {
        let ray = lhs;
        let plane = rhs;

        if plane.distance(ray.origin()) > ray.reach {
            return Self::None;
        }
        
        match LinePlaneIntersection::intersection(ray.line, plane) {
            LinePlaneIntersection::FromInside { point } => Self::FromInside { point },
            LinePlaneIntersection::FromOutside { point } => Self::FromOutside { point },
            LinePlaneIntersection::OnBoundary => Self::OnBoundary,
            LinePlaneIntersection::None => Self::None,
        }
        
    }
}

pub enum LineBspIntersection {}

impl<'l, 'r> Intersection<'l, 'r> for LineBspIntersection {
    // The direction and length to go.
    type Lhs = (Line, f32);

    type Rhs = &'r MapBsp;

    fn intersection(lhs: Self::Lhs, rhs: Self::Rhs) -> Self {
        todo!()
    }
}

//pub enum RayBspIntersection {
//    None {
//        empty_mats: Vec<Handle<PhysMat>>,
//    },
//    Collide {
//        empty_mats: Vec<Handle<PhysMat>>,
//        solid_mat: Handle<PhysMat>,
//        normal: Vec3,
//        point: Vec3,
//    },
//    Trapped {
//        solid_mat: Vec<Handle<PhysMat>>,
//    },
//}
//
//impl<'l, 'r> Intersection<'l, 'r> for RayBspIntersection {
//    type Rhs = &'r MapBsp;
//    type Lhs = Line;
//
//    fn intersection(lhs: Self::Lhs, rhs: Self::Rhs) -> Self {
//        let (mut line, bsp) = (lhs, rhs);
//        let mut stack = vec![bsp.root_node_id()];
//        let mut empty_mats = vec![];
//        let mut last_point = Vec3::ZERO;
//        let last_normal = Vec3::ZERO;
//        while let Some(node) = stack.pop().and_then(|id| bsp.get_node(id)) {
//            match *node {
//                BspNode::EmptyLeaf(mat) => empty_mats.push(mat),
//                BspNode::SolidLeaf(mat) => {
//                    return Self::Collide {
//                        empty_mats,
//                        solid_mat: mat,
//                        normal: last_normal,
//                        point: last_point,
//                    };
//                }
//                BspNode::Parent {
//                    plane_id,
//                    front,
//                    back,
//                } => match RayPlaneIntersection::intersection(line, bsp.get_plane(plane_id)) {
//                    RayPlaneIntersection::FromInside { point } => {
//                        last_point = point;
//                        stack.push(back);
//                        stack.push(front);
//                    }
//                    RayPlaneIntersection::FromOutside { point } => {
//                        last_point = point;
//                        stack.push(back);
//                        stack.push(front);
//                    }
//                    RayPlaneIntersection::OnBoundary => {
//                        last_point = line.origin;
//                    }
//                    RayPlaneIntersection::None => {
//                        stack.push(front);
//                    }
//                },
//            }
//        }
//        unreachable!()
//    }
//}

pub enum BspPointIntersection {
    Inside { solid_mat: Handle<PhysMat> },
    Outside { empty_mat: Handle<PhysMat> },
}

impl<'l, 'r> Intersection<'l, 'r> for BspPointIntersection {
    type Lhs = &'l MapBsp;

    type Rhs = Vec3;

    fn intersection(lhs: Self::Lhs, rhs: Self::Rhs) -> Self {
        let (bsp, point) = (lhs, rhs);
        let mut stack = vec![bsp.root_node_id()];

        while let Some(node) = stack.pop().and_then(|node_id| bsp.get_node(node_id)) {
            match *node {
                BspNode::EmptyLeaf(empty_mat) => {
                    return Self::Outside { empty_mat };
                }
                BspNode::SolidLeaf(solid_mat) => {
                    return Self::Inside { solid_mat };
                }
                BspNode::Parent {
                    plane_id,
                    front,
                    back,
                } => match PointPlaneIntersection::intersection(point, bsp.get_plane(plane_id)) {
                    PointPlaneIntersection::Inside => stack.push(back),
                    PointPlaneIntersection::OnBoundary | PointPlaneIntersection::Outside => {
                        stack.push(front)
                    }
                },
            }
        }
        unreachable!()
    }
}

pub enum PlanePlaneIntersection {
    Line(Line),
    None,
}

impl<'l, 'r> Intersection<'l, 'r> for PlanePlaneIntersection {
    type Lhs = Plane;

    type Rhs = Plane;

    fn intersection(lhs: Self::Lhs, rhs: Self::Rhs) -> Self {
        // The direction of the line intersection is normal to both the normal of the two planes.
        let dir = lhs.normal.cross(rhs.normal);
        let det = dir.length_squared();

        // Check if planes are not parallel.
        if det < f32::EPSILON {
            return Self::None;
        };
        // Its safe to normalize because the planes are not parallel.
        let dir = dir.normalize();

        // Calculate the origin for the line.
        let ld = -lhs.origin.dot(lhs.normal);
        let rd = -rhs.origin.dot(rhs.normal);
        let angle = lhs.normal.dot(rhs.normal);
        let origin = lhs.normal * (ld - rd * angle) + rhs.normal * (rd - ld * angle);

        Self::Line(Line {
            dir,
            origin: (dir.cross(lhs.normal) * ld + rhs.normal.cross(dir) * rd / dir.length()),
        })
    }
}
