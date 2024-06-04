use bevy::input::ButtonInput;

use crate::input::Axis;

pub trait ButtonInputExt<T> {
    fn axis(&self, pos: T, neg: T) -> Axis;
}

impl<T: Copy + Eq + PartialEq + Send + Sync + std::hash::Hash> ButtonInputExt<T> for ButtonInput<T> {
    fn axis(&self, pos: T, neg: T) -> Axis {
        let mut _pos= 0.0;
        let mut _neg = 0.0;
        if self.pressed(pos) {
            _pos = 1.0;
        } 
        if  self.pressed(neg) {
            _neg = 1.0
        } 
        Axis {
            pos: _pos,
            neg: _neg,
        }
    }
}

