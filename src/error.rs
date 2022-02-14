use async_tungstenite::tungstenite;
use rmp_serde::{encode, decode};

pub type LighthouseResult<T> = Result<T, LighthouseError>;

/// The type for any error involved in communication with the lighthouse.
#[derive(Debug)]
pub enum LighthouseError {
    Tungstenite(tungstenite::Error),
    Encode(encode::Error),
    Decode(decode::Error),
    Server { code: i32, message: Option<String> },
    Custom(String),
}

impl LighthouseError {
    /// Creates a new `LighthouseError` from the given custom message.
    pub fn custom(s: &str) -> Self { Self::Custom(s.to_owned()) }
}

impl From<tungstenite::Error> for LighthouseError {
    fn from(e: tungstenite::Error) -> Self { Self::Tungstenite(e) }
}

impl From<encode::Error> for LighthouseError {
    fn from(e: encode::Error) -> Self { Self::Encode(e) }
}

impl From<decode::Error> for LighthouseError {
    fn from(e: decode::Error) -> Self { Self::Decode(e) }
}
