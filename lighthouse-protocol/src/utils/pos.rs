use std::{fmt, ops::{Add, Sub, AddAssign, SubAssign}};

use crate::Delta;

/// A position on the integer grid.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
}

impl Pos {
    /// The origin.
    pub const ZERO: Self = Self::new(0, 0);

    /// Creates a mew position.
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl fmt::Display for Pos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Add<Delta> for Pos {
    type Output = Pos;

    fn add(self, rhs: Delta) -> Self {
        Self::new(self.x + rhs.dx, self.y + rhs.dy)
    }
}

impl Sub<Pos> for Pos {
    type Output = Delta;

    fn sub(self, rhs: Self) -> Delta {
        Delta::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Sub<Delta> for Pos {
    type Output = Pos;

    fn sub(self, rhs: Delta) -> Self {
        self + (-rhs)
    }
}

impl AddAssign<Delta> for Pos {
    fn add_assign(&mut self, rhs: Delta) {
        *self = *self + rhs;
    }
}

impl SubAssign<Delta> for Pos {
    fn sub_assign(&mut self, rhs: Delta) {
        *self = *self - rhs;
    }
}
