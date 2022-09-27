use std::ops::Range;

use rand::{Rng, seq::IteratorRandom, thread_rng};

use crate::{Pos, Delta, LIGHTHOUSE_COLS};

/// A rectangle on the integer grid.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Rect {
    pub origin: Pos,
    pub size: Delta,
}

impl Rect {
    /// Creates a new rectangle.
    pub const fn new(origin: Pos, size: Delta) -> Self {
        Self { origin, size }
    }

    /// The range of x values.
    pub const fn x_range(self) -> Range<i32> {
        self.origin.x..(self.origin.x + self.size.dx)
    }

    /// The range of y values.
    pub const fn y_range(self) -> Range<i32> {
        self.origin.y..(self.origin.y + self.size.dy)
    }

    /// Checks whether the rectangle contains the given position.
    pub const fn contains(self, pos: Pos) -> bool {
        pos.x >= self.origin.x && pos.x < self.origin.x + self.width()
        && pos.y >= self.origin.y && pos.y < self.origin.y + self.height()
    }

    /// Converts a position to an index.
    pub fn index_of(self, pos: Pos) -> usize {
        debug_assert!(self.contains(pos));
        let relative = pos - self.origin;
        relative.dy as usize * LIGHTHOUSE_COLS + relative.dx as usize
    }

    /// Whether this rectangle is empty.
    pub const fn is_empty(self) -> bool {
        self.size.dx == 0 && self.size.dy == 0
    }

    /// Samples a random position within the rectangle with the given rng.
    pub fn sample_random_with(self, rng: &mut impl Rng) -> Option<Pos> {
        let x = self.x_range().choose(rng)?;
        let y = self.y_range().choose(rng)?;
        Some(Pos::new(x, y))
    }

    /// Samples a random position within the rectangle.
    pub fn sample_random(self) -> Option<Pos> {
        self.sample_random_with(&mut thread_rng())
    }

    /// The rectangle's width.
    pub const fn width(self) -> i32 {
        self.size.dx
    }

    /// The rectangle's height.
    pub const fn height(self) -> i32 {
        self.size.dy
    }

    /// The rectangle's area.
    pub const fn area(self) -> i32 {
        self.width() * self.height()
    }

    /// Wraps a value to the rectangle's bounds.
    pub const fn wrap(self, pos: Pos) -> Pos {
        Pos::new(
            pos.x.rem_euclid(self.width()),
            pos.y.rem_euclid(self.height()),
        )
    }
}
