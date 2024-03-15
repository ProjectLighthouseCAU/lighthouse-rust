use async_tungstenite::tungstenite;
use rmp_serde::{encode, decode};

pub type Result<T> = std::result::Result<T, Error>;

/// The type for any error involved in communication with the lighthouse.
#[derive(Debug)]
pub enum Error {
    Tungstenite(tungstenite::Error),
    Encode(encode::Error),
    Decode(decode::Error),
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

impl From<encode::Error> for Error {
    fn from(e: encode::Error) -> Self { Self::Encode(e) }
}

impl From<decode::Error> for Error {
    fn from(e: decode::Error) -> Self { Self::Decode(e) }
}
