use serde::{Serialize, Deserialize};

use crate::Authentication;

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Payload {
    // TODO
    Display()
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientMessage {
    #[serde(rename = "REID")]
    request_id: i32,
    #[serde(rename = "VERB")]
    verb: String,
    #[serde(rename = "PATH")]
    path: Vec<String>,
    #[serde(rename = "AUTH")]
    authentication: Authentication,
    #[serde(rename = "PAYL")]
    payload: Payload
}
