#![no_std]

extern crate alloc;

use core::ops::{SubAssign, AddAssign};

pub mod user;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Dimensions<V>(V, V);

impl<V> Dimensions<V> {
    pub fn set_width(&mut self, value: V) {
        self.0 = value;
    }

    pub fn set_height(&mut self, value: V) {
        self.1 = value;
    }
}

impl<V: Clone + Copy> Dimensions<V> {
    pub fn inner(self) -> (V, V) {
        (self.0, self.1)
    }

    pub fn width(&self) -> V {
        self.0
    }

    pub fn height(&self) -> V {
        self.1
    }
}

impl<V: Default> Default for Dimensions<V> {
    fn default() -> Self {
        Self(V::default(), V::default())
    }
}



#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position<V>(V, V);

impl<V> Position<V> {
    pub fn set_x(&mut self, value: V) {
        self.0 = value;
    }

    pub fn set_y(&mut self, value: V) {
        self.1 = value;
    }
}

impl<V: Clone + Copy> Position<V> {
    pub fn inner(self) -> (V, V) {
        (self.0, self.1)
    }

    pub fn x(&self) -> V {
        self.0
    }

    pub fn y(&self) -> V {
        self.1
    }
}

impl<V: SubAssign<V> + AddAssign<V>> Position<V> {
    pub fn inc_x(&mut self, value: V) {
        self.0 += value;
    }

    pub fn dec_x(&mut self, value: V) {
        self.0 -= value;
    }

    pub fn inc_y(&mut self, value: V) {
        self.1 += value;
    }

    pub fn dec_y(&mut self, value: V) {
        self.1 -= value;
    }
}

impl<V: Default> Default for Position<V> {
    fn default() -> Self {
        Self(V::default(), V::default())
    }
}