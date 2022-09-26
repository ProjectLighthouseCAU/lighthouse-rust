use std::{fmt, ops::{Add, Sub, AddAssign, SubAssign}};

use rand::{prelude::Distribution, distributions::Standard};

use crate::{LIGHTHOUSE_COLS, LIGHTHOUSE_ROWS, Delta};

/// A position on the lighthouse display.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
}

impl Pos {
    /// Creates a mew position. The parameters must be in bounds.
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    /// Whether the position is in range.
    pub fn in_range(self) -> bool {
        self.x >= 0 && self.y >= 0 && self.x < LIGHTHOUSE_COLS as i32 && self.y < LIGHTHOUSE_ROWS as i32
    }

    /// Adds a delta to this position, wrapping around.
    pub fn add_wrapping(self, rhs: Delta) -> Self {
        Self::new(
            (self.x + rhs.dx).rem_euclid(LIGHTHOUSE_COLS as i32),
            (self.y + rhs.dy).rem_euclid(LIGHTHOUSE_ROWS as i32),
        )
    }

    /// Subtracts a delta from this position, wrapping around.
    pub fn sub_wrapping(self, rhs: Delta) -> Self {
        self.add_wrapping(-rhs)
    }
}

impl Distribution<Pos> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Pos {
        Pos::new(rng.gen_range(0..LIGHTHOUSE_COLS as i32), rng.gen_range(0..LIGHTHOUSE_ROWS as i32))
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
