use async_tungstenite::tungstenite;
use lighthouse_protocol::ValueError;

pub type Result<T> = std::result::Result<T, Error>;

/// The type for any error involved in communication with the lighthouse.
#[derive(Debug)]
pub enum Error {
    Tungstenite(tungstenite::Error),
    Encode(rmp_serde::encode::Error),
    Decode(rmp_serde::decode::Error),
    Value(ValueError),
    Server { code: i32, message: Option<String>, warnings: Vec<String> },
    Custom(String),
}

impl Error {
    /// Creates a new `LighthouseError` from the given custom message.
    pub fn custom(s: &str) -> Self { Self::Custom(s.to_owned()) }
}

impl From<tungstenite::Error> for Error {
    fn from(e: tungstenite::Error) -> Self { Self::Tungstenite(e) }
}

impl From<rmp_serde::encode::Error> for Error {
    fn from(e: rmp_serde::encode::Error) -> Self { Self::Encode(e) }
}

impl From<rmp_serde::decode::Error> for Error {
    fn from(e: rmp_serde::decode::Error) -> Self { Self::Decode(e) }
}

impl From<ValueError> for Error {
    fn from(e: ValueError) -> Self { Self::Value(e) }
}
