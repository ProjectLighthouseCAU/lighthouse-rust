use std::{fmt::Debug, ops::Neg};

use rand::{prelude::Distribution, distributions::Standard};
use serde::{Deserialize, Serialize};

use super::{Vec2, Unity, Zero};

/// One of the four cardinal directions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn approximate_from<T>(vec2: Vec2<T>) -> Option<Self> where T: Zero + Unity + PartialEq + Neg<Output = T> + PartialOrd + Copy {
        if vec2 == Vec2::ZERO {
            return None;
        }

        // See https://www.desmos.com/calculator/472pdoxzqa for visualization
        // Note that the y-axis is flipped here, per computer graphics conventions,
        // hence the sign flip (-y instead of y).
        let Vec2 { x, y } = vec2;
        let left_or_up = x < -y;
        let right_or_up = -x < -y;
        Some(
            match (left_or_up, right_or_up) {
                (true, true) => Direction::Up,
                (true, false) => Direction::Left,
                (false, true) => Direction::Right,
                (false, false) => Direction::Down, 
            }
        )
    }
}

impl<T> TryFrom<Vec2<T>> for Direction where T: Zero + Unity + PartialEq + Debug {
    type Error = String;

    fn try_from(vec2: Vec2<T>) -> Result<Self, Self::Error> {
        if vec2 == Vec2::UP {
            Ok(Direction::Up)
        } else if vec2 == Vec2::DOWN {
            Ok(Direction::Down)
        } else if vec2 == Vec2::LEFT {
            Ok(Direction::Left)
        } else if vec2 == Vec2::RIGHT {
            Ok(Direction::Right)
        } else {
            Err(format!("Not a direction: {:?}", vec2))
        }
    }
}

impl<T> From<Direction> for Vec2<T> where T: Zero + Unity {
    fn from(direction: Direction) -> Self {
        match direction {
            Direction::Up => Vec2::UP,
            Direction::Down => Vec2::DOWN,
            Direction::Left => Vec2::LEFT,
            Direction::Right => Vec2::RIGHT,
        }
    }
}

impl Distribution<Direction> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Direction {
        match rng.gen_range(0..4) {
            0 => Direction::Up,
            1 => Direction::Down,
            2 => Direction::Left,
            3 => Direction::Right,
            _ => unreachable!(),
        }
    }
}
