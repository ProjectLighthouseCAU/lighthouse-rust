use serde::{Serialize, Deserialize};

use crate::{Authentication, Meta, Value, ValueError, Verb};

/// A message originating from the lighthouse client.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct ClientMessage<P> {
    #[serde(rename = "REID")]
    pub request_id: i32,
    #[serde(rename = "VERB")]
    pub verb: Verb,
    #[serde(rename = "PATH")]
    pub path: Vec<String>,
    #[serde(rename = "META")]
    pub meta: Meta,
    #[serde(rename = "AUTH")]
    pub authentication: Authentication,
    #[serde(rename = "PAYL")]
    pub payload: P,
}

impl ClientMessage<Value> {
    pub fn decode_payload<R>(self) -> Result<ClientMessage<R>, ValueError>
    where
        R: for<'de> Deserialize<'de> {
        Ok(ClientMessage {
            request_id: self.request_id,
            verb: self.verb,
            path: self.path,
            meta: self.meta,
            authentication: self.authentication,
            payload: rmpv::ext::from_value(self.payload)?,
        })
    }
}
