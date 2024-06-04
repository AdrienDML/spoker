use std::ops::Index;

pub struct Arring<const C: usize, T> {
    data: [T; C],
    start: usize,
    end: usize,
}

impl<const C: usize, T> Arring<C, T> {
    pub const fn cap() -> usize {
        C
    }

    pub fn len(&self) -> usize {
        if self.start < self.end {
            self.end - self.start
        } else {
            C - self.start + self.end
        }
    }
    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }

    pub fn get(&self, idx: usize) -> Option<&T> {
        if idx < self.len() {
            Some(&self.data[(self.start + idx) % C])
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, idx: usize) -> Option<&mut T> {
        if idx < self.len() {
            Some(&mut self.data[(self.start + idx) % C])
        } else {
            None
        }
    }
}

impl<const C: usize, T> Index<usize> for Arring<C, T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.get(index).unwrap()
    }
}
