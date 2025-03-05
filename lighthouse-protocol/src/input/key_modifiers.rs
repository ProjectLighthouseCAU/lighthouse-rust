use serde::{Deserialize, Serialize};

/// A keyboard event.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct KeyModifiers {
    /// Whether the alt key is held.
    pub alt: bool,
    /// Whether the ctrl key is held.
    pub ctrl: bool,
    /// Whether the meta key is held.
    pub meta: bool,
    /// Whether the shiftKey key is held.
    pub shift: bool,
}

impl Default for KeyModifiers {
    fn default() -> Self {
        Self {
            alt: false,
            ctrl: false,
            meta: false,
            shift: false,
        }
    }
}
