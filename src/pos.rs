use std::{fmt, ops::{Add, Sub, AddAssign, Neg, SubAssign}};

use rand::{prelude::Distribution, distributions::Standard, thread_rng, Rng};

use crate::{LIGHTHOUSE_COLS, LIGHTHOUSE_ROWS};

/// A position on the lighthouse display.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Pos {
    pub x: usize,
    pub y: usize,
}

/// A delta on the lighthouse display.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Delta {
    pub dx: i32,
    pub dy: i32,
}

impl Pos {
    /// Creates a mew position. The parameters must be in bounds.
    pub fn new(x: usize, y: usize) -> Self {
        assert!(x < LIGHTHOUSE_COLS);
        assert!(y < LIGHTHOUSE_ROWS);
        Self { x, y }
    }
}

impl Delta {
    /// Creates a new vector.
    pub fn new(dx: i32, dy: i32) -> Self {
        Self { dx, dy }
    }

    /// Randomly one of the four cardinal directions.
    pub fn random_direction() -> Self {
        let random_offset = || { if thread_rng().gen() { 1 } else { -1 } };
        if thread_rng().gen() {
            Self::new(0, random_offset())
        } else {
            Self::new(random_offset(), 0)
        }
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

impl fmt::Display for Delta {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.dx, self.dy)
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

impl Add for Delta {
    type Output = Delta;

    fn add(self, rhs: Self) -> Self {
        Self::new(self.dx + rhs.dx, self.dy + rhs.dy)
    }
}

impl Sub for Delta {
    type Output = Delta;

    fn sub(self, rhs: Self) -> Self {
        Self::new(self.dx - rhs.dx, self.dy - rhs.dy)
    }
}

impl Neg for Delta {
    type Output = Delta;

    fn neg(self) -> Self {
        Self::new(-self.dx, -self.dy)
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
