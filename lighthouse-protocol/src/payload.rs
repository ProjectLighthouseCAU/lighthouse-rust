use serde::{Serialize, Deserialize};
use serde_with::rust::deserialize_ignore_any;

use crate::{Frame, InputEvent};

/// The payload of a message.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(untagged)]
pub enum Payload {
    Frame(Frame),
    InputEvent(InputEvent),
    #[serde(deserialize_with = "deserialize_ignore_any")]
    Empty,
}
