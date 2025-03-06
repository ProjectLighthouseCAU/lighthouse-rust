use serde::{Deserialize, Serialize};

use crate::Direction;

use super::{EventSource, GamepadControlEvent};

/// A gamepad/controller event.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GamepadEvent {
    /// The client identifier. Also unique per gamepad.
    pub source: EventSource,
    /// The control-specific info.
    #[serde(flatten)]
    pub control: GamepadControlEvent,
}

impl GamepadEvent {
    /// Parses the gamepad event as an arbitrary direction.
    pub fn direction(&self) -> Option<Direction> {
        self.left_direction().or_else(|| self.right_direction())
    }

    /// The direction if the gamepad event represents a D-pad or left stick.
    /// Commonly used e.g. for movement in games.
    pub fn left_direction(&self) -> Option<Direction> {
        match &self.control {
            GamepadControlEvent::Button(button) => button.d_pad_direction(),
            GamepadControlEvent::Axis2D(axis2d) if axis2d.index == 0 => axis2d.direction(),
            _ => None,
        }
    }

    /// The direction if the gamepad event represents a right stick event.
    /// Commonly used e.g. for camera control in games.
    pub fn right_direction(&self) -> Option<Direction> {
        match &self.control {
            GamepadControlEvent::Axis2D(axis2d) if axis2d.index == 1 => axis2d.direction(),
            _ => None,
        }
    }
}
