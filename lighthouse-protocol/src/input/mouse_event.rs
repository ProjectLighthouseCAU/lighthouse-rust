use serde::{Deserialize, Serialize};

use super::{EventSource, MouseButton};

/// A mouse event.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct MouseEvent {
    /// The client identifier.
    pub source: EventSource,
    /// The mouse button.
    pub button: MouseButton,
    // TODO: Add pos
}
