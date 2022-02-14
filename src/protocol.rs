use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use serde_with::rust::deserialize_ignore_any;

use crate::{Authentication, Display};

/// A key/controller input event.
#[derive(Debug, Serialize, Deserialize)]
pub struct InputEvent {
    #[serde(rename = "src")]
    pub source: i32,
    pub key: Option<i32>,
    #[serde(rename = "btn")]
    pub button: Option<i32>,
    #[serde(rename = "dwn")]
    pub is_down: bool,
}

/// The payload of a message.
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Payload {
    Display(Display),
    InputEvent(InputEvent),
    #[serde(deserialize_with = "deserialize_ignore_any")]
    Empty,
}

/// A message originating from the lighthouse client.
#[derive(Debug, Serialize, Deserialize)]
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

/// A message originating from the lighthouse server.
#[derive(Debug, Serialize, Deserialize)]
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
