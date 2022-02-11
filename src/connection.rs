use async_tungstenite::{async_std::{connect_async, ConnectStream}, WebSocketStream};
use crate::{Authentication, LighthouseResult};

pub struct Connection {
    authentication: Authentication,
    connection: WebSocketStream<ConnectStream>
}

impl Connection {
    pub async fn new(authentication: Authentication) -> LighthouseResult<Self> {
        Ok(Self {
            authentication,
            connection: connect_async("wss://lighthouse.uni-kiel.de/websocket").await?.0,
        })
    }
}
