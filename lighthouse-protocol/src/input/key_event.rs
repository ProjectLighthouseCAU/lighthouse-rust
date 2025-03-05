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
    /// Whether the event is a repeat event.
    pub repeat: bool,
    /// The key pressed, see the docs on JS's `KeyboardEvent.code` for details.
    pub code: String, // TODO: Extract stronger `Key` type
    /// Whether the alt key is held.
    pub alt_key: bool,
    /// Whether the ctrl key is held.
    pub ctrl_key: bool,
    /// Whether the meta key is held.
    pub meta_key: bool,
    /// Whether the shiftKey key is held.
    pub shift_key: bool,
}
