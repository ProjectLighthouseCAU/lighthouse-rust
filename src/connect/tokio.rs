use async_tungstenite::{WebSocketStream, tokio::{ConnectStream, connect_async}};

use crate::{Result, Lighthouse, Authentication, LIGHTHOUSE_URL, TokioSpawner};

impl Lighthouse<WebSocketStream<ConnectStream>> {
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
