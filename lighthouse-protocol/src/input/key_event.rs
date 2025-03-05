use serde::{Deserialize, Serialize};

use super::{EventSource, KeyModifiers};

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
    /// The held key modifiers.
    pub modifiers: KeyModifiers,
}
