use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use crate::{Authentication, Payload};

/// A message originating from the lighthouse client.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct ClientMessage {
    #[serde(rename = "REID")]
    pub request_id: i32,
    #[serde(rename = "VERB")]
    pub verb: String,
    #[serde(rename = "PATH")]
    pub path: Vec<String>,
    #[serde(rename = "META")]
    pub meta: HashMap<String, String>,
    #[serde(rename = "AUTH")]
    pub authentication: Authentication,
    #[serde(rename = "PAYL")]
    pub payload: Payload,
}
