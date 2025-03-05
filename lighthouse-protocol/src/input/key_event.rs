use serde::{Deserialize, Serialize};

use crate::{Delta, Unity, Zero};

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

impl KeyEvent {
    /// The direction if either the WASD or arrow keys were pressed.
    pub fn direction<T>(&self) -> Option<Delta<T>> where T: Zero + Unity {
        self.wasd_direction().or_else(|| self.arrow_direction())
    }

    /// The direction if one of the WASD keys was pressed.
    pub fn wasd_direction<T>(&self) -> Option<Delta<T>> where T: Zero + Unity {
        match self.code.as_str() {
            "KeyW" => Some(Delta::UP),
            "KeyA" => Some(Delta::LEFT),
            "KeyS" => Some(Delta::DOWN),
            "KeyD" => Some(Delta::RIGHT),
            _ => None,
        }
    }

    /// The direction if one of the arrow keys was pressed.
    pub fn arrow_direction<T>(&self) -> Option<Delta<T>> where T: Zero + Unity {
        match self.code.as_str() {
            "ArrowUp" => Some(Delta::UP),
            "ArrowLeft" => Some(Delta::LEFT),
            "ArrowDown" => Some(Delta::DOWN),
            "ArrowRight" => Some(Delta::RIGHT),
            _ => None,
        }
    }
}
