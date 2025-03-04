use serde::{Deserialize, Serialize};

/// A mouse button.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub enum MouseButton {
    Left,
    Middle,
    Right,
    #[serde(untagged)]
    Unknown(String),
}
