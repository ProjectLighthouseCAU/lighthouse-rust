use serde::{Deserialize, Serialize};

use crate::Pos;

use super::{EventSource, MouseButton};

/// A mouse event.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MouseEvent {
    /// The client identifier.
    pub source: EventSource,
    /// The mouse button.
    pub button: MouseButton,
    /// The mouse position.
    pub pos: Pos<f64>,
}
