use serde::{Deserialize, Serialize};

use crate::Vec2;

/// A 2D axis event on a gamepad.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(tag = "control", rename_all = "camelCase")]
pub struct GamepadAxis2DEvent {
    /// The 2D axes index (0 is the left stick, 1 is the right stick).
    pub index: usize,
    /// The value of the axis (each component is between -1.0 and 1.0, modeled after the Web Gamepad API).
    pub value: Vec2<f64>,
}
