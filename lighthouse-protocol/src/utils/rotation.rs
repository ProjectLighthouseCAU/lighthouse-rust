use std::ops::{Add, Mul, Neg};

use rand::{thread_rng, Rng};

use crate::Vec2;

use super::{Unity, Zero};

// TODO: Rename this to Mat2?

/// An 2D rotation that is representable using an integer matrix.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Rotation<T> {
    /// The integer matrix representing the corresponding linear transformation.
    matrix: [T; 4],
}

impl<T> Rotation<T> where T: Zero + Unity + Neg<Output = T> {
    /// The identity rotation.
    pub const IDENTITY: Self = Self::new([
        T::ONE,  T::ZERO,
        T::ZERO, T::ONE,
    ]);
    /// The rotation by 90° clockwise.
    pub const CW_90: Self = Self::new([
         T::ZERO, T::NEG_ONE,
         T::ONE,  T::ZERO,
    ]);
    /// The rotation by 180° clockwise or counter-clockwise.
    pub const CW_180: Self = Self::new([
        T::NEG_ONE, T::ZERO,
        T::ZERO,    T::NEG_ONE,
    ]);
    /// The rotation by 270° clockwise (or 90° counter-clockwise).
    pub const CW_270: Self = Self::new([
        T::ZERO,    T::ONE,
        T::NEG_ONE, T::ZERO,
    ]);

    /// Creates a new rotation from the given matrix.
    pub const fn new(matrix: [T; 4]) -> Self {
        Self { matrix }
    }

    /// Randomly one of the four cardinal rotations with the given rng.
    pub fn random_cardinal_with(rng: &mut impl Rng) -> Self {
        match rng.gen_range(0..4) {
            0 => Self::IDENTITY,
            1 => Self::CW_90,
            2 => Self::CW_180,
            3 => Self::CW_270,
            _ => unreachable!(),
        }
    }

    /// Randomly one of the four cardinal rotations with the thread-local rng.
    pub fn random_cardinal() -> Self {
        Self::random_cardinal_with(&mut thread_rng())
    }
}

impl<T> Mul<Self> for Rotation<T> where T: Zero + Unity + Neg<Output = T> + Add<Output = T> + Mul<Output = T> + Copy {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        // Standard 2x2 matrix multiplication
        Self::new([
            self.matrix[0] * rhs.matrix[0] + self.matrix[1] * rhs.matrix[2],
            self.matrix[0] * rhs.matrix[1] + self.matrix[1] * rhs.matrix[3],
            self.matrix[2] * rhs.matrix[0] + self.matrix[3] * rhs.matrix[2],
            self.matrix[2] * rhs.matrix[1] + self.matrix[3] * rhs.matrix[3],
        ])
    }
}

impl<T> Mul<Vec2<T>> for Rotation<T> where T: Zero + Unity + Neg<Output = T> + Add<Output = T> + Mul<Output = T> + Copy {
    type Output = Vec2<T>;

    fn mul(self, rhs: Vec2<T>) -> Vec2<T> {
        // Standard matrix-vector multiplication
        Vec2::new(
            self.matrix[0] * rhs.x + self.matrix[1] * rhs.y,
            self.matrix[2] * rhs.x + self.matrix[3] * rhs.y ,
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::Vec2;

    use super::Rotation;

    #[test]
    fn rotation() {
        assert_eq!(Rotation::IDENTITY * Vec2::new(4, -3), Vec2::new(4, -3));
        assert_eq!(Rotation::CW_90 * Vec2::new(2, 3), Vec2::new(-3, 2));
        assert_eq!(Rotation::CW_90 * Vec2::<i32>::RIGHT, Vec2::DOWN);
        assert_eq!(Rotation::CW_90 * Vec2::<i32>::DOWN, Vec2::LEFT);
        assert_eq!(Rotation::CW_90 * Vec2::<i32>::LEFT, Vec2::UP);
        assert_eq!(Rotation::CW_90 * Vec2::<i32>::UP, Vec2::RIGHT);
    }

    #[test]
    fn matmul() {
        assert_eq!(Rotation::IDENTITY * Rotation::<i32>::IDENTITY, Rotation::IDENTITY);
        assert_eq!(Rotation::IDENTITY * Rotation::<i32>::CW_90, Rotation::CW_90);
        assert_eq!(Rotation::CW_90 * Rotation::<i32>::CW_90, Rotation::CW_180);
        assert_eq!(Rotation::CW_90 * Rotation::<i32>::CW_180, Rotation::CW_270);
        assert_eq!(Rotation::CW_180 * Rotation::<i32>::CW_180, Rotation::IDENTITY);
    }
}
