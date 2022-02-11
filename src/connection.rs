use async_tungstenite::{async_std::{connect_async, ConnectStream}, WebSocketStream};
use crate::{LighthouseAuth, LighthouseResult};

pub struct LighthouseConnection {
    auth: LighthouseAuth,
    connection: WebSocketStream<ConnectStream>
}

impl LighthouseConnection {
    pub async fn new(auth: LighthouseAuth) -> LighthouseResult<Self> {
        Ok(Self {
            auth,
            connection: connect_async("wss://lighthouse.uni-kiel.de/websocket").await?.0,
        })
    }
}
