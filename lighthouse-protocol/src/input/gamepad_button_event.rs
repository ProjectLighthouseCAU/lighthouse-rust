use serde::{Deserialize, Serialize};

/// A button event on a gamepad.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(tag = "control", rename_all = "camelCase")]
pub struct GamepadButtonEvent {
    /// The button index.
    index: usize,
    /// Whether the button is pressed.
    down: bool,
    /// The value of the button (between 0.0 and 1.0, modeled after the Web Gamepad API).
    value: f64,
}
