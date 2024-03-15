use async_tungstenite::{WebSocketStream, tokio::{ConnectStream, connect_async}};
use lighthouse_protocol::Authentication;

use crate::{Result, Lighthouse, LIGHTHOUSE_URL, TokioSpawner};

pub type TokioWebSocket = WebSocketStream<ConnectStream>;

impl Lighthouse<TokioWebSocket> {
    /// Connects to the lighthouse server at the given URL.
    pub async fn connect_with_tokio_to(url: &str, authentication: Authentication) -> Result<Self> {
        let (web_socket, _) = connect_async(url).await?;
        Self::new::<TokioSpawner>(web_socket, authentication)
    }

    /// Connects to the lighthouse server at the default URL.
    pub async fn connect_with_tokio(authentication: Authentication) -> Result<Self> {
        Self::connect_with_tokio_to(LIGHTHOUSE_URL, authentication).await
    }
}
