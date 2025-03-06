use serde::{Deserialize, Serialize};

use crate::{Direction, Vec2};

/// A 2D axis event on a gamepad.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(tag = "control", rename_all = "camelCase")]
pub struct GamepadAxis2DEvent {
    /// The 2D axes index (0 is the left stick, 1 is the right stick).
    pub index: usize,
    /// The value of the axis (each component is between -1.0 and 1.0, modeled after the Web Gamepad API).
    pub value: Vec2<f64>,
}

impl GamepadAxis2DEvent {
    /// The approximate direction (outside of a small deadzone).
    pub fn direction(&self) -> Option<Direction> {
        let deadzone_radius: f64 = 0.1;
        if self.value.length() < deadzone_radius {
            return None;
        }

        // See https://www.desmos.com/calculator/472pdoxzqa for visualization
        // Note that the y-axis is flipped here, per computer graphics conventions,
        // hence the sign flip (-y instead of y).
        let Vec2 { x, y } = self.value;
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

#[cfg(test)]
mod tests {
    use crate::{Direction, Vec2, Zero};

    use super::GamepadAxis2DEvent;

    #[test]
    fn directions() {
        assert_eq!(event(Vec2::UP).direction(), Some(Direction::Up));
        assert_eq!(event(Vec2::DOWN).direction(), Some(Direction::Down));
        assert_eq!(event(Vec2::LEFT).direction(), Some(Direction::Left));
        assert_eq!(event(Vec2::RIGHT).direction(), Some(Direction::Right));
        assert_eq!(event(Vec2::ZERO).direction(), None);
        assert_eq!(event(Vec2::new(-0.05, 0.05)).direction(), None); // within deadzone
    }

    fn event(value: Vec2<f64>) -> GamepadAxis2DEvent {
        GamepadAxis2DEvent {
            index: 0,
            value,
        }
    }
}
