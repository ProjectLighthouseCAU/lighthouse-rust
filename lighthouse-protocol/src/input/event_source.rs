use std::fmt;

use serde::{Deserialize, Serialize};

/// An identifier that is unique per client + device combo.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Hash)]
#[serde(untagged)]
pub enum EventSource {
    String(String),
    Int(i32),
}

impl fmt::Display for EventSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EventSource::String(s) => write!(f, "{s}"),
            EventSource::Int(i) => write!(f, "{i}"),
        }
    }
}
