use serde::{Serialize, Deserialize};

/// A key/controller input event.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
pub struct InputEvent {
    #[serde(rename = "src")]
    pub source: i32,
    pub key: Option<i32>,
    #[serde(rename = "btn")]
    pub button: Option<i32>,
    #[serde(rename = "dwn")]
    pub is_down: bool,
}
