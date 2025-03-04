use serde::{Deserialize, Serialize};

/// An identifier that is unique per client + device combo.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum EventSource {
    String(String),
    Int(i32),
}
