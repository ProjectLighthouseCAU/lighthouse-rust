use serde::{Deserialize, Serialize};

use super::Zero;

/// A 3D rotation.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Rot3<T> {
    pub alpha: T,
    pub beta: T,
    pub gamma: T,
}

impl<T> Rot3<T> {
    /// Creates a mew position.
    pub const fn new(alpha: T, beta: T, gamma: T) -> Self {
        Self { alpha, beta, gamma }
    }

    /// Maps a function over the vector.
    pub fn map<U>(self, mut f: impl FnMut(T) -> U) -> Rot3<U> {
        Rot3 {
            alpha: f(self.alpha),
            beta: f(self.beta),
            gamma: f(self.gamma),
        }
    }
}

impl<T> Zero for Rot3<T> where T: Zero {
    /// The origin.
    const ZERO: Self = Self::new(T::ZERO, T::ZERO, T::ZERO);
}
