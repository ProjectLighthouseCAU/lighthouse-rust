use serde::{Deserialize, Serialize};

use super::EventSource;

/// An unknown event.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UnknownEvent {
    /// The event type.
    #[serde(rename = "type")]
    pub event_type: String,
    /// The client identifier. Also unique per gamepad.
    pub source: EventSource,
}
