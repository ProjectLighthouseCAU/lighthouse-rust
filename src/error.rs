use async_tungstenite::tungstenite;
use rmp_serde::encode;

pub type LighthouseResult<T> = Result<T, LighthouseError>;

#[derive(Debug)]
pub enum LighthouseError {
    Tungstenite(tungstenite::Error),
    Encode(encode::Error),
}

impl From<tungstenite::Error> for LighthouseError {
    fn from(e: tungstenite::Error) -> Self { Self::Tungstenite(e) }
}

impl From<encode::Error> for LighthouseError {
    fn from(e: encode::Error) -> Self { Self::Encode(e) }
}
