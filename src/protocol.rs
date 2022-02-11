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
    #[serde(rename = "AUTH")]
    pub authentication: Authentication,
    #[serde(rename = "PAYL")]
    pub payload: Payload
}
