use std::{collections::HashMap, fmt};

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

impl fmt::Display for DirectoryTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        let count = self.entries.len();
        for (i, (key, value)) in self.entries.iter().enumerate() {
            write!(f, "\"{}\": ", key)?;
            if let Some(value) = value {
                write!(f, "{}", value)?;
            } else {
                write!(f, "None")?;
            }
            if i < count - 1 {
                write!(f, ", ")?;
            }
        }
        write!(f, "}}")?;
        Ok(())
    }
}
