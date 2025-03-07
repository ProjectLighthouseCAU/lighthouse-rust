use serde::{Deserialize, Serialize};

use crate::{Delta, Pos};

use super::{EventSource, MouseButton};

/// A mouse event.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MouseEvent {
    /// The client identifier.
    pub source: EventSource,
    /// Whether the button was pressed.
    pub down: bool,
    /// Whether the mouse pointer was locked (e.g. to the frontend's canvas)
    pub pointer_locked: bool,
    /// The mouse button.
    pub button: MouseButton,
    /// The mouse position on the lighthouse grid.
    pub pos: Pos<f64>,
    /// The mouse movement on the lighthouse grid.
    pub movement: Delta<f64>,
}
