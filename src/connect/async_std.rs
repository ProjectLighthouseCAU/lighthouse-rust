use async_tungstenite::{WebSocketStream, async_std::{ConnectStream, connect_async}};

use crate::{LighthouseResult, Lighthouse, Authentication, LIGHTHOUSE_URL};

impl Lighthouse<WebSocketStream<ConnectStream>> {
    /// Connects to the lighthouse server at the given URL.
    pub async fn connect_with_async_std_to(url: &str, authentication: Authentication) -> LighthouseResult<Self> {
        let (web_socket, _) = connect_async(url).await?;
        Self::new(web_socket, authentication)
    }

    /// Connects to the lighthouse server at the default URL.
    pub async fn connect_with_async_std(authentication: Authentication) -> LighthouseResult<Self> {
        Self::connect_with_async_std_to(LIGHTHOUSE_URL, authentication).await
    }
}
