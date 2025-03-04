use std::{fmt::Debug, ops::{Add, Mul, Range, Sub}};

use rand::{Rng, seq::IteratorRandom, thread_rng};

use crate::{Vec2, LIGHTHOUSE_COLS};

use super::{RemEuclid, Zero};

/// A rectangle on the integer grid.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Rect<T> {
    pub origin: Vec2<T>,
    pub size: Vec2<T>,
}

impl<T> Rect<T> {
    /// Creates a new rectangle.
    pub const fn new(origin: Vec2<T>, size: Vec2<T>) -> Self {
        Self { origin, size }
    }
}

impl<T> Rect<T> where T: Copy {
    /// The rectangle's width.
    pub const fn width(self) -> T {
        self.size.x
    }

    /// The rectangle's height.
    pub const fn height(self) -> T {
        self.size.y
    }
}

impl<T> Rect<T> where T: Mul<Output = T> + Copy {
    /// The rectangle's area.
    pub fn area(self) -> T {
        self.width() * self.height()
    }
}

impl<T> Rect<T> where T: RemEuclid + Copy {
    /// Wraps a value to the rectangle's bounds.
    pub fn wrap(self, pos: Vec2<T>) -> Vec2<T> {
        Vec2::new(
            pos.x.rem_euclid(self.width()),
            pos.y.rem_euclid(self.height()),
        )
    }
}

impl<T> Rect<T> where T: Zero + Eq + Copy {
    /// Whether this rectangle is empty.
    pub fn is_empty(self) -> bool {
        self.size.x == T::ZERO && self.size.y == T::ZERO
    }
}

impl<T> Rect<T> where T: Add<Output = T> + Copy {
    /// The range of x values.
    pub fn x_range(self) -> Range<T> {
        self.origin.x..(self.origin.x + self.size.x)
    }

    /// The range of y values.
    pub fn y_range(self) -> Range<T> {
        self.origin.y..(self.origin.y + self.size.y)
    }
}

impl<T> Rect<T> where T: Add<Output = T> + Copy, Range<T>: IteratorRandom<Item = T> {
    /// Samples a random position within the rectangle with the given rng.
    pub fn sample_random_with(self, rng: &mut impl Rng) -> Option<Vec2<T>> {
        let x = self.x_range().choose(rng)?;
        let y = self.y_range().choose(rng)?;
        Some(Vec2::<T>::new(x, y))
    }

    /// Samples a random position within the rectangle.
    pub fn sample_random(self) -> Option<Vec2<T>> {
        self.sample_random_with(&mut thread_rng())
    }
}

impl<T> Rect<T> where T: Add<Output = T> + Ord + Copy {
    /// Checks whether the rectangle contains the given position.
    pub fn contains(self, pos: Vec2<T>) -> bool {
        pos.x >= self.origin.x && pos.x < self.origin.x + self.width()
        && pos.y >= self.origin.y && pos.y < self.origin.y + self.height()
    }
}

impl<T> Rect<T> where T: Add<Output = T> + Sub<Output = T> + TryInto<usize> + Ord + Copy, T::Error: Debug {
    /// Converts a position to an index.
    pub fn index_of(self, pos: Vec2<T>) -> usize {
        debug_assert!(self.contains(pos));
        let relative = pos - self.origin;
        relative.y.try_into().unwrap() * LIGHTHOUSE_COLS + relative.x.try_into().unwrap()
    }
}
