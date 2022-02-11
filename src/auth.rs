#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LighthouseAuth {
    username: String,
    token: String,
}

impl LighthouseAuth {
    pub fn new(username: &str, token: &str) -> Self {
        Self {
            username: username.to_owned(),
            token: token.to_owned(),
        }
    }

    pub fn username(&self) -> &str { self.username.as_str() }

    pub fn token(&self) -> &str { self.token.as_str() }
}
