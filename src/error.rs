use async_tungstenite::tungstenite;

pub type LighthouseResult<T> = Result<T, LighthouseError>;

pub enum LighthouseError {
    Tungstenite(tungstenite::Error),
}

impl From<tungstenite::Error> for LighthouseError {
    fn from(e: tungstenite::Error) -> Self { Self::Tungstenite(e) }
}
