use std::{fmt, ops::{Add, Sub, Neg}};

use rand::{Rng, thread_rng};

use crate::Rotation;

/// A 2D vector on the lighthouse display.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Delta {
    pub dx: i32,
    pub dy: i32,
}

impl Delta {
    /// The vector pointing one pixel to the left.
    pub const LEFT:  Self = Self::new(-1,  0);
    /// The vector pointing one pixel up.
    pub const UP:    Self = Self::new( 0, -1);
    /// The vector pointing one pixel to the right.
    pub const RIGHT: Self = Self::new( 1,  0);
    /// The vector pointing one pixel down.
    pub const DOWN:  Self = Self::new( 0,  1);

    /// Creates a new vector.
    pub const fn new(dx: i32, dy: i32) -> Self {
        Self { dx, dy }
    }

    /// Randomly one of the four cardinal directions with the given rng.
    pub fn random_cardinal_with(rng: &mut impl Rng) -> Self {
        Rotation::random_cardinal_with(rng) * Self::RIGHT
    }

    /// Randomly one of the four cardinal directions with the thread-local rng.
    pub fn random_cardinal() -> Self {
        Self::random_cardinal_with(&mut thread_rng())
    }
}

impl fmt::Display for Delta {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.dx, self.dy)
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
