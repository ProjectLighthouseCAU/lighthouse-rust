use serde::{Deserialize, Serialize};

use crate::Direction;

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
    pub fn direction(&self) -> Option<Direction> {
        self.wasd_direction().or_else(|| self.arrow_direction())
    }

    /// The direction if one of the WASD keys was pressed.
    pub fn wasd_direction(&self) -> Option<Direction> {
        match self.code.as_str() {
            "KeyW" => Some(Direction::Up),
            "KeyA" => Some(Direction::Left),
            "KeyS" => Some(Direction::Down),
            "KeyD" => Some(Direction::Right),
            _ => None,
        }
    }

    /// The direction if one of the arrow keys was pressed.
    pub fn arrow_direction(&self) -> Option<Direction> {
        match self.code.as_str() {
            "ArrowUp" => Some(Direction::Up),
            "ArrowLeft" => Some(Direction::Left),
            "ArrowDown" => Some(Direction::Down),
            "ArrowRight" => Some(Direction::Right),
            _ => None,
        }
    }
}
