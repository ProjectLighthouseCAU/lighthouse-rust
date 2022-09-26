use std::{fmt, ops::{Add, Sub, Neg}};

use rand::{thread_rng, Rng};

/// A 2D vector on the lighthouse display.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Delta {
    pub dx: i32,
    pub dy: i32,
}

impl Delta {
    /// Creates a new vector.
    pub const fn new(dx: i32, dy: i32) -> Self {
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
