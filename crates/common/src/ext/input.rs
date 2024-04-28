use bevy::input::ButtonInput;

pub trait ButtonInputExt<T> {
    fn axis(&self, pos: T, neg: T) -> f32;
}

impl<T: Copy + Eq + PartialEq + Send + Sync + std::hash::Hash> ButtonInputExt<T> for ButtonInput<T> {
    fn axis(&self, pos: T, neg: T) -> f32 {
        if self.pressed(pos) && !self.pressed(neg) {
            1.0
        } else if !self.pressed(pos) && self.pressed(neg) {
            -1.0
        } else {
            0.0
        }
    }
}

