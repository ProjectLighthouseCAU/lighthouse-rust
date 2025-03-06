use serde::{Deserialize, Serialize};

use crate::Direction;

/// A button event on a gamepad.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(tag = "control", rename_all = "camelCase")]
pub struct GamepadButtonEvent {
    /// The button index.
    pub index: usize,
    /// Whether the button is pressed.
    pub down: bool,
    /// The value of the button (between 0.0 and 1.0, modeled after the Web Gamepad API).
    pub value: f64,
}

impl GamepadButtonEvent {
    /// The direction if one of the D-pad buttons was pressed.
    /// See https://www.w3.org/TR/gamepad/#dfn-standard-gamepad
    pub fn d_pad_direction(&self) -> Option<Direction> {
        match self.index {
            12 => Some(Direction::Up),
            13 => Some(Direction::Down),
            14 => Some(Direction::Left),
            15 => Some(Direction::Right),
            _ => None,
        }
    }
}
