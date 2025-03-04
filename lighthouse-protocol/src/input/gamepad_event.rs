use serde::{Deserialize, Serialize};

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
