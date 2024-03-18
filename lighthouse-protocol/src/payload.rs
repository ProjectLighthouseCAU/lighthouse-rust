use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use crate::{Frame, InputEvent};

/// The payload of a model message.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(untagged)]
pub enum Model {
    Frame(Frame),
    InputEvent(InputEvent),
}

/// The payload of a LIST request.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(transparent)]
pub struct DirectoryTree {
    pub entries: HashMap<String, Option<DirectoryTree>>,
}
