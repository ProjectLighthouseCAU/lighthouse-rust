use serde::{Deserialize, Serialize};

use super::EventSource;

/// A keyboard event.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct KeyEvent {
    /// The client identifier.
    pub source: EventSource,
    /// Whether the key was pressed.
    pub down: bool,
    /// The key pressed, see the docs on JS's `KeyboardEvent.key` for details.
    pub key: String, // TODO: Extract stronger `Key` type
}
