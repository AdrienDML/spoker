use bevy::math::Vec3;

pub trait Vec3Ext: Sized {
    fn null_x(self) -> Self;
    fn null_y(self) -> Self;
    fn null_z(self) -> Self;

    #[inline]
    fn null_xy(self) -> Self {
        self.null_x().null_y()
    }
    #[inline]
    fn null_xz(self) -> Self {
        self.null_x().null_z()
    }
    #[inline]
    fn null_yz(self) -> Self {
        self.null_y().null_z()
    }

    fn flip_x(self) -> Self;
    fn flip_y(self) -> Self;
    fn flip_z(self) -> Self;

    #[inline]
    fn flip_xy(self) -> Self {
        self.flip_x().flip_y()
    }
    #[inline]
    fn flip_xz(self) -> Self {
        self.flip_x().flip_z()
    }
    #[inline]
    fn flip_yz(self) -> Self {
        self.flip_y().flip_z()
    }

    #[inline]
    fn flip(self) -> Self {
        self.flip_x().flip_y().flip_z()
    }

    fn in_triangle(self, a: Vec3, b: Vec3, c: Vec3) -> bool;

    fn intersect_plane(self, normal: Vec3, point: Vec3) -> bool;

    fn dist_dir(self) -> Option<(f32, Vec3)>;

    fn signed_angle_in_plane(self, normal: Vec3, other: Vec3) -> f32;
}

impl Vec3Ext for Vec3 {
    #[inline]
    fn null_x(mut self) -> Self {
        self.x = 0.0;
        self
    }

    #[inline]
    fn null_y(mut self) -> Self {
        self.y = 0.0;
        self
    }

    #[inline]
    fn null_z(mut self) -> Self {
        self.z = 0.0;
        self
    }

    #[inline]
    fn flip_x(mut self) -> Self {
        self.x = -self.x;
        self
    }

    #[inline]
    fn flip_y(mut self) -> Self {
        self.y = -self.y;
        self
    }

    #[inline]
    fn flip_z(mut self) -> Self {
        self.z = -self.z;
        self
    }

    #[inline]
    fn in_triangle(self, a: Vec3, b: Vec3, c: Vec3) -> bool {
        let inv_area = 1.0 / (b - a).cross(c - a).length();
        let alpha = (self - a).cross(self - b).length() * inv_area;
        let beta = (self - b).cross(self - c).length() * inv_area;
        let gamma = (self - c).cross(self - a).length() * inv_area;

        (0.0..1.0).contains(&alpha)
            && (0.0..1.0).contains(&beta)
            && (0.0..1.0).contains(&gamma)
            && ((alpha + beta + gamma) - 1.0).abs() < f32::EPSILON
    }

    fn intersect_plane(self, normal: Vec3, point: Vec3) -> bool {
        (self - point).dot(normal) < 0.0
    }

    fn dist_dir(self) -> Option<(f32, Vec3)> { 
        let l = self.length();
        if l > f32::EPSILON {
            Some((l, self / l))
        } else {
            None
        }
    }

    fn signed_angle_in_plane(self, normal: Vec3, other: Vec3) -> f32 {
        self.cross(other).dot(normal).atan2(self.dot(other))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_in_triangle() {
        let a = Vec3::new(10.0, 0.0, 10.0);
        let b = Vec3::new(-10.0, 0.0, 10.0);
        let c = Vec3::new(-10.0, 0.0, -10.0);

        let ins = Vec3::ZERO;
        let out = Vec3::new(10.0, 0.0, -10.0);

        assert!(ins.in_triangle(a, b, c));
        assert!(!out.in_triangle(a, b, c));
    }
}
