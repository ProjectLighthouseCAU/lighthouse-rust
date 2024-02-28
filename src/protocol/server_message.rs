use serde::{Serialize, Deserialize};

use crate::{Payload, Result, Error};

/// A message originating from the lighthouse server.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct ServerMessage {
    #[serde(rename = "RNUM")]
    pub code: i32,
    #[serde(rename = "REID")]
    pub request_id: Option<i32>,
    #[serde(rename = "WARNINGS", skip_serializing_if = "Vec::is_empty", default)]
    pub warnings: Vec<String>,
    #[serde(rename = "RESPONSE")]
    pub response: Option<String>,
    #[serde(rename = "PAYL")]
    pub payload: Payload,
}

impl ServerMessage {
    /// Checks this response and returns only if successful.
    pub fn check(self) -> Result<Self> {
        if self.code == 200 {
            Ok(self)
        } else {
            Err(Error::Server { code: self.code, message: self.response, warnings: self.warnings })
        }
    }
}
