use async_tungstenite::tungstenite;
use lighthouse_protocol::ValueError;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

/// The type for any error involved in communication with the lighthouse.
#[derive(Debug, Error)]
pub enum Error {
    #[error("Tungstenite (WebSocket) error: {0}")]
    Tungstenite(#[from] tungstenite::Error),
    #[error("MessagePack encoding error: {0}")]
    Encode(#[from] rmp_serde::encode::Error),
    #[error("MessagePack decoding error: {0}")]
    Decode(#[from] rmp_serde::decode::Error),
    #[error("MessagePack value error: {0}")]
    Value(#[from] ValueError),
    #[error("Server error: {} {} (warnings: {:?})", code, message.clone().unwrap_or_else(|| "(no message)".to_string()), warnings)]
    Server { code: i32, message: Option<String>, warnings: Vec<String> },
    #[error("No next message available")]
    NoNextMessage,
    #[error("The connection was closed")]
    ConnectionClosed,
    #[error("Custom error")]
    Custom(String),
}

impl Error {
    /// Creates a new `LighthouseError` from the given custom message.
    pub fn custom(s: &str) -> Self { Self::Custom(s.to_owned()) }
}
