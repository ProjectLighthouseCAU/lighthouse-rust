use serde::{Deserialize, Serialize};

/// A 1D axis event on a gamepad.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(tag = "control", rename_all = "camelCase")]
pub struct GamepadAxisEvent {
    /// The 1D axis index.
    pub index: usize,
    /// The value of the axis (between -1.0 and 1.0, modeled after the Web Gamepad API).
    pub value: f64,
}
