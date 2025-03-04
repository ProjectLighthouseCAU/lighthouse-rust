use serde::{Serialize, Deserialize};

use crate::{Frame, LegacyInputEvent};

/// The payload of a model message.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(untagged)]
pub enum Model {
    Frame(Frame),
    InputEvent(LegacyInputEvent),
}
