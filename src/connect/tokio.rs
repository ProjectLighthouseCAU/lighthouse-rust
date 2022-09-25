use async_tungstenite::{WebSocketStream, tokio::{ConnectStream, connect_async}};

use crate::{LighthouseResult, Lighthouse, Authentication, LIGHTHOUSE_URL};

/// Connects to the lighthouse server at the given URL.
pub async fn connect_to(url: &str, authentication: Authentication) -> LighthouseResult<Lighthouse<WebSocketStream<ConnectStream>>> {
    let (web_socket, _) = connect_async(url).await?;
    Lighthouse::new(web_socket, authentication)
}

/// Connects to the lighthouse server at the default URL.
pub async fn connect(authentication: Authentication) -> LighthouseResult<Lighthouse<WebSocketStream<ConnectStream>>> {
    connect_to(LIGHTHOUSE_URL, authentication).await
}
