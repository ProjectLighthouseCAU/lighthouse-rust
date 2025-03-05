use serde::{Deserialize, Serialize};

use super::{GamepadAxis2DEvent, GamepadAxisEvent, GamepadButtonEvent};

/// A control-specific event on a gamepad.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(tag = "control", rename_all = "camelCase")]
pub enum GamepadControlEvent {
    Button(GamepadButtonEvent),
    Axis(GamepadAxisEvent),
    #[serde(rename = "axis2d")]
    Axis2D(GamepadAxis2DEvent),
}
