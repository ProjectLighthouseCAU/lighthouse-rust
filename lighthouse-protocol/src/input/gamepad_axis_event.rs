use serde::{Deserialize, Serialize};

/// An axis event on a gamepad.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(tag = "control", rename_all = "camelCase")]
pub struct GamepadAxisEvent {
    /// The axis index.
    index: usize,
    /// The value of the axis (between -1.0 and 1.0, modeled after the Web Gamepad API).
    value: f64,
}
