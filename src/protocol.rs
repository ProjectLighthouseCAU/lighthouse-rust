use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use crate::{Authentication, Display};

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Payload {
    Display(Display),
}

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
    // TODO: Support other cases
    // #[serde(rename = "PAYL")]
    // pub payload: Payload,
}