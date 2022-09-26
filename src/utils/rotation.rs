use std::ops::Mul;

use rand::{thread_rng, Rng};

use crate::Delta;

/// An 2D rotation that is representable using an integer matrix.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Rotation {
    /// The integer matrix representing the corresponding linear transformation.
    matrix: [i32; 4],
}

impl Rotation {
    /// The identity rotation.
    pub const IDENTITY: Self = Self::new([
         1,  0,
         0,  1,
    ]);
    /// The rotation by 90° clockwise.
    pub const CW_90: Self = Self::new([
         0, -1,
         1,  0,
    ]);
    /// The rotation by 180° clockwise or counter-clockwise.
    pub const CW_180: Self = Self::new([
        -1,  0,
         0, -1,
    ]);
    /// The rotation by 270° clockwise (or 90° counter-clockwise).
    pub const CW_270: Self = Self::new([
         0,  1,
        -1,  0,
    ]);

    /// Creates a new rotation from the given matrix.
    pub const fn new(matrix: [i32; 4]) -> Self {
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

impl Mul<Rotation> for Rotation {
    type Output = Rotation;

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

impl Mul<Delta> for Rotation {
    type Output = Delta;

    fn mul(self, rhs: Delta) -> Delta {
        // Standard matrix-vector multiplication
        Delta::new(
            self.matrix[0] * rhs.dx + self.matrix[1] * rhs.dy,
            self.matrix[2] * rhs.dx + self.matrix[3] * rhs.dy ,
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::Delta;

    use super::Rotation;

    #[test]
    fn test_rotation() {
        assert_eq!(Rotation::IDENTITY * Delta::new(4, -3), Delta::new(4, -3));
        assert_eq!(Rotation::CW_90 * Delta::new(2, 3), Delta::new(-3, 2));
        assert_eq!(Rotation::CW_90 * Delta::RIGHT, Delta::DOWN);
        assert_eq!(Rotation::CW_90 * Delta::DOWN, Delta::LEFT);
        assert_eq!(Rotation::CW_90 * Delta::LEFT, Delta::UP);
        assert_eq!(Rotation::CW_90 * Delta::UP, Delta::RIGHT);
    }

    #[test]
    fn test_matmul() {
        assert_eq!(Rotation::IDENTITY * Rotation::IDENTITY, Rotation::IDENTITY);
        assert_eq!(Rotation::IDENTITY * Rotation::CW_90, Rotation::CW_90);
        assert_eq!(Rotation::CW_90 * Rotation::CW_90, Rotation::CW_180);
        assert_eq!(Rotation::CW_90 * Rotation::CW_180, Rotation::CW_270);
        assert_eq!(Rotation::CW_180 * Rotation::CW_180, Rotation::IDENTITY);
    }
}
