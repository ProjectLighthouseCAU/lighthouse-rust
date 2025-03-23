use std::{fmt, ops::{Add, AddAssign, Mul, Neg, Sub, SubAssign}};

use serde::{Deserialize, Serialize};

use super::{Sqrt, Zero};

/// A 3D vector.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vec3<T> {
    /// Creates a mew position.
    pub const fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    /// Maps a function over the vector.
    pub fn map<U>(self, mut f: impl FnMut(T) -> U) -> Vec3<U> {
        Vec3 {
            x: f(self.x),
            y: f(self.y),
            z: f(self.z),
        }
    }
}

impl<T> Zero for Vec3<T> where T: Zero {
    /// The origin.
    const ZERO: Self = Self::new(T::ZERO, T::ZERO, T::ZERO);
}

impl<T> Vec3<T> where T: Add<Output = T> + Mul<Output = T> + Sqrt + Copy {
    /// The vector's length.
    pub fn length(&self) -> T {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
}

impl<T> fmt::Display for Vec3<T> where T: fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl<T> Add for Vec3<T> where T: Add<Output = T> {
    type Output = Self;

    fn add(self, rhs: Vec3<T>) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl<T> Neg for Vec3<T> where T: Neg<Output = T> {
    type Output = Self;

    fn neg(self) -> Self {
        Self::new(-self.x, -self.y, -self.z)
    }
}

impl<T> Sub for Vec3<T> where T: Sub<Output = T> {
    type Output = Self;

    fn sub(self, rhs: Vec3<T>) -> Self {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl<T> AddAssign<Self> for Vec3<T> where T: AddAssign<T> {
    fn add_assign(&mut self, rhs: Vec3<T>) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl<T> SubAssign<Self> for Vec3<T> where T: SubAssign<T> {
    fn sub_assign(&mut self, rhs: Vec3<T>) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}
