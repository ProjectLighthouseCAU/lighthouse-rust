use serde::{Deserialize, Serialize};

use super::EventSource;

/// A gamepad/controller event.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GamepadEvent {
    /// The client identifier. Also unique per gamepad.
    pub source: EventSource,
    // TODO: Add remaining
}
