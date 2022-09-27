use std::ops::Range;

use crate::{Pos, Delta};

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
