use std::{fmt, ops::{Add, AddAssign, Neg, Sub, SubAssign}};

use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};

use super::{Unity, Zero};

/// A 2D vector.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vec2<T> {
    /// Creates a mew position.
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T> Zero for Vec2<T> where T: Zero {
    /// The origin.
    const ZERO: Self = Self::new(T::ZERO, T::ZERO);
}

impl<T> Vec2<T> where T: Zero + Unity {
    /// The vector pointing one pixel to the left.
    pub const LEFT:  Self = Self::new(T::NEG_ONE, T::ZERO);
    /// The vector pointing one pixel up.
    pub const UP:    Self = Self::new(T::ZERO, T::NEG_ONE);
    /// The vector pointing one pixel to the right.
    pub const RIGHT: Self = Self::new(T::ONE, T::ZERO);
    /// The vector pointing one pixel down.
    pub const DOWN:  Self = Self::new(T::ZERO, T::ONE);

    /// Randomly one of the four cardinal rotations with the given rng.
    pub fn random_cardinal_with(rng: &mut impl Rng) -> Self {
        match rng.gen_range(0..4) {
            0 => Self::LEFT,
            1 => Self::UP,
            2 => Self::RIGHT,
            3 => Self::DOWN,
            _ => unreachable!(),
        }
    }

    /// Randomly one of the four cardinal rotations with the thread-local rng.
    pub fn random_cardinal() -> Self {
        Self::random_cardinal_with(&mut thread_rng())
    }
}

impl<T> fmt::Display for Vec2<T> where T: fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl<T> Add for Vec2<T> where T: Add<Output = T> {
    type Output = Self;

    fn add(self, rhs: Vec2<T>) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T> Neg for Vec2<T> where T: Neg<Output = T> {
    type Output = Self;

    fn neg(self) -> Self {
        Self::new(-self.x, -self.y)
    }
}

impl<T> Sub for Vec2<T> where T: Sub<Output = T> {
    type Output = Self;

    fn sub(self, rhs: Vec2<T>) -> Self {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<T> AddAssign<Self> for Vec2<T> where T: AddAssign<T> {
    fn add_assign(&mut self, rhs: Vec2<T>) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T> SubAssign<Self> for Vec2<T> where T: SubAssign<T> {
    fn sub_assign(&mut self, rhs: Vec2<T>) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

/// A type alias that semantically expresses a position.
pub type Pos<T> = Vec2<T>;

/// A type alias that semantically expresses an offset/delta.
pub type Delta<T> = Vec2<T>;
