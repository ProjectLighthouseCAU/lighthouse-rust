use std::fmt::Debug;

use rand::{prelude::Distribution, distributions::Standard};
use serde::{Deserialize, Serialize};

use super::{Delta, Unity, Zero};

/// One of the four cardinal directions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl<T> TryFrom<Delta<T>> for Direction where T: Zero + Unity + PartialEq + Debug {
    type Error = String;

    fn try_from(delta: Delta<T>) -> Result<Self, Self::Error> {
        if delta == Delta::UP {
            Ok(Direction::Up)
        } else if delta == Delta::DOWN {
            Ok(Direction::Down)
        } else if delta == Delta::LEFT {
            Ok(Direction::Left)
        } else if delta == Delta::RIGHT {
            Ok(Direction::Right)
        } else {
            Err(format!("Not a direction: {:?}", delta))
        }
    }
}

impl<T> From<Direction> for Delta<T> where T: Zero + Unity {
    fn from(direction: Direction) -> Self {
        match direction {
            Direction::Up => Delta::UP,
            Direction::Down => Delta::DOWN,
            Direction::Left => Delta::LEFT,
            Direction::Right => Delta::RIGHT,
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
