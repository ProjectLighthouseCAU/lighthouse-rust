use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Authentication {
    #[serde(rename = "USER")]
    pub username: String,
    #[serde(rename = "TOKEN")]
    pub token: String,
}

impl Authentication {
    pub fn new(username: &str, token: &str) -> Self {
        Self {
            username: username.to_owned(),
            token: token.to_owned(),
        }
    }
}
