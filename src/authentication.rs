use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Authentication {
    #[serde(rename = "USER")]
    username: String,
    #[serde(rename = "TOKEN")]
    token: String,
}

impl Authentication {
    pub fn new(username: &str, token: &str) -> Self {
        Self {
            username: username.to_owned(),
            token: token.to_owned(),
        }
    }

    pub fn username(&self) -> &str { self.username.as_str() }

    pub fn token(&self) -> &str { self.token.as_str() }
}
