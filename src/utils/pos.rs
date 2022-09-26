use std::{fmt, ops::{Add, Sub, AddAssign, SubAssign}};

use rand::{prelude::Distribution, distributions::Standard};

use crate::{LIGHTHOUSE_COLS, LIGHTHOUSE_ROWS, Delta};

/// A position on the lighthouse display.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Pos {
    pub x: usize,
    pub y: usize,
}

impl Pos {
    /// Creates a mew position. The parameters must be in bounds.
    pub const fn new(x: usize, y: usize) -> Self {
        assert!(x < LIGHTHOUSE_COLS);
        assert!(y < LIGHTHOUSE_ROWS);
        Self { x, y }
    }
}

impl Distribution<Pos> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Pos {
        Pos::new(rng.gen_range(0..LIGHTHOUSE_COLS), rng.gen_range(0..LIGHTHOUSE_ROWS))
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
        Self::new(
            (self.x as i32 + rhs.dx).rem_euclid(LIGHTHOUSE_COLS as i32) as usize,
            (self.y as i32 + rhs.dy).rem_euclid(LIGHTHOUSE_ROWS as i32) as usize,
        )
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
