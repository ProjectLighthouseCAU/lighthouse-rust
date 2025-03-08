use std::{fmt::Debug, ops::{Add, Div, Mul, Range, Sub}};

use rand::{Rng, seq::IteratorRandom, thread_rng};

use crate::{Vec2, LIGHTHOUSE_COLS};

use super::{RemEuclid, Unity, Zero};

/// A rectangle on the integer grid.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Rect<T> {
    pub origin: Vec2<T>,
    pub size: Vec2<T>,
}

impl<T> Rect<T> {
    /// Creates a new rectangle.
    pub const fn new(origin: Vec2<T>, size: Vec2<T>) -> Self {
        Self { origin, size }
    }
}

impl<T> Rect<T> where T: Copy {
    /// The rectangle's width.
    pub const fn width(self) -> T {
        self.size.x
    }

    /// The rectangle's height.
    pub const fn height(self) -> T {
        self.size.y
    }

    /// Fetches the top-left corner of the rectangle, i.e. the origin.
    pub const fn top_left(self) -> Vec2<T> {
        self.origin
    }
}

impl<T> Rect<T> where T: Add<Output = T> + Copy {
    /// Fetches the bottom-right corner of the rectangle.
    pub fn bottom_right(self) -> Vec2<T> {
        self.origin + self.size
    }
}

impl<T> Rect<T> where T: Add<Output = T> + Copy + Zero {
    /// Fetches the bottom-left corner of the rectangle.
    pub fn bottom_left(self) -> Vec2<T> {
        self.origin + Vec2::new(T::ZERO, self.size.y)
    }

    /// Fetches the top-right corner of the rectangle.
    pub fn top_right(self) -> Vec2<T> {
        self.origin + Vec2::new(self.size.x, T::ZERO)
    }
}

impl<T> Rect<T> where T: Add<Output = T> + Div<Output = T> + Copy + Zero + Unity {
    /// The generic constant 2.
    fn two() -> T {
        T::ONE + T::ONE
    }

    /// Fetches the top-center position of the rectangle.
    pub fn top_center(self) -> Vec2<T> {
        self.origin + Vec2::new(self.size.x / Self::two(), T::ZERO)
    }

    /// Fetches the left-center position of the rectangle.
    pub fn center_left(self) -> Vec2<T> {
        self.origin + Vec2::new(T::ZERO, self.size.y / Self::two())
    }

    /// Fetches the right-center position of the rectangle.
    pub fn center_right(self) -> Vec2<T> {
        self.origin + Vec2::new(self.size.x, self.size.y / Self::two())
    }

    /// Fetches the bottom-center position of the rectangle.
    pub fn bottom_center(self) -> Vec2<T> {
        self.origin + Vec2::new(self.size.x / Self::two(), self.size.y)
    }

    /// Fetches the center position of the rectangle.
    pub fn center(self) -> Vec2<T> {
        self.origin + self.size.map(|c| c / Self::two())
    }
}

impl<T> Rect<T> where T: Mul<Output = T> + Copy {
    /// The rectangle's area.
    pub fn area(self) -> T {
        self.width() * self.height()
    }
}

impl<T> Rect<T> where T: RemEuclid + Copy {
    /// Wraps a value to the rectangle's bounds.
    pub fn wrap(self, pos: Vec2<T>) -> Vec2<T> {
        Vec2::new(
            pos.x.rem_euclid(self.width()),
            pos.y.rem_euclid(self.height()),
        )
    }
}

impl<T> Rect<T> where T: Zero + Eq + Copy {
    /// Whether this rectangle is empty.
    pub fn is_empty(self) -> bool {
        self.size.x == T::ZERO && self.size.y == T::ZERO
    }
}

impl<T> Rect<T> where T: Add<Output = T> + Copy {
    /// The range of x values.
    pub fn x_range(self) -> Range<T> {
        self.origin.x..(self.origin.x + self.size.x)
    }

    /// The range of y values.
    pub fn y_range(self) -> Range<T> {
        self.origin.y..(self.origin.y + self.size.y)
    }
}

impl<T> Rect<T> where T: Add<Output = T> + Copy, Range<T>: IteratorRandom<Item = T> {
    /// Samples a random position within the rectangle with the given rng.
    pub fn sample_random_with(self, rng: &mut impl Rng) -> Option<Vec2<T>> {
        let x = self.x_range().choose(rng)?;
        let y = self.y_range().choose(rng)?;
        Some(Vec2::<T>::new(x, y))
    }

    /// Samples a random position within the rectangle.
    pub fn sample_random(self) -> Option<Vec2<T>> {
        self.sample_random_with(&mut thread_rng())
    }
}

impl<T> Rect<T> where T: Add<Output = T> + Ord + Copy {
    /// Checks whether the rectangle contains the given position.
    pub fn contains(self, pos: Vec2<T>) -> bool {
        pos.x >= self.origin.x && pos.x < self.origin.x + self.width()
        && pos.y >= self.origin.y && pos.y < self.origin.y + self.height()
    }

    /// Checks whether the rectangle intersects the given rectangle.
    pub fn intersects(self, other: Rect<T>) -> bool {
        let s1 = self.top_left();
        let e1 = self.bottom_right();
        let s2 = other.top_left();
        let e2 = other.bottom_right();
        s2.x < e1.x && s1.x < e2.x &&
        s2.y < e1.y && s1.y < e2.y
    }
}

impl<T> Rect<T> where T: Add<Output = T> + Sub<Output = T> + TryInto<usize> + Ord + Copy, T::Error: Debug {
    /// Converts a position to an index.
    pub fn index_of(self, pos: Vec2<T>) -> usize {
        debug_assert!(self.contains(pos));
        let relative = pos - self.origin;
        relative.y.try_into().unwrap() * LIGHTHOUSE_COLS + relative.x.try_into().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Sub;

    use crate::Vec2;

    use super::Rect;

    #[test]
    fn unit_points() {
        let rect = Rect::new(Vec2::new(-1, -1), Vec2::new(2, 2));
        assert_eq!(rect.top_left(), Vec2::new(-1, -1));
        assert_eq!(rect.top_center(), Vec2::new(0, -1));
        assert_eq!(rect.top_right(), Vec2::new(1, -1));

        assert_eq!(rect.center_left(), Vec2::new(-1, 0));
        assert_eq!(rect.center(), Vec2::new(0, 0));
        assert_eq!(rect.center_right(), Vec2::new(1, 0));

        assert_eq!(rect.bottom_left(), Vec2::new(-1, 1));
        assert_eq!(rect.bottom_center(), Vec2::new(0, 1));
        assert_eq!(rect.bottom_right(), Vec2::new(1, 1));
    }

    #[test]
    fn intersections() {
        assert!(rect(0, 0, 2, 2).intersects(rect(1, 1, 3, 3)));
        assert!(rect(0, 0, 2, 2).intersects(rect(1, -1, 1, 3)));
        assert!(!rect(0, -2, 1, 1).intersects(rect(1, -1, 1, 3)));
        assert!(!rect(0, 0, 1, 1).intersects(rect(1, 0, 2, 1)));
        assert!(rect(0, 0, 2, 1).intersects(rect(1, 0, 2, 1)));
        assert!(!rect(0, 0, 2, 0).intersects(rect(1, 0, 2, 1)));
    }

    fn rect<T>(sx: T, sy: T, ex: T, ey: T) -> Rect<T> where T: Copy + Sub<Output = T> {
        Rect::new(Vec2::new(sx, sy), Vec2::new(ex - sx, ey - sy))
    }
}
