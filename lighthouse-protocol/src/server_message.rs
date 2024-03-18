use serde::{Serialize, Deserialize};
use crate::{Value, ValueError};

/// A message originating from the lighthouse server.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct ServerMessage<P> {
    #[serde(rename = "RNUM")]
    pub code: i32,
    #[serde(rename = "REID")]
    pub request_id: Option<i32>,
    #[serde(rename = "WARNINGS", skip_serializing_if = "Vec::is_empty", default)]
    pub warnings: Vec<String>,
    #[serde(rename = "RESPONSE")]
    pub response: Option<String>,
    #[serde(rename = "PAYL")]
    pub payload: P,
}

impl ServerMessage<Value> {
    pub fn decode_payload<R>(self) -> Result<ServerMessage<R>, ValueError>
    where
        R: for<'de> Deserialize<'de> {
        Ok(ServerMessage {
            code: self.code,
            request_id: self.request_id,
            warnings: self.warnings,
            response: self.response,
            payload: rmpv::ext::from_value(self.payload)?,
        })
    }
}
