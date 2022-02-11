use async_tungstenite::{async_std::{connect_async, ConnectStream}, WebSocketStream, tungstenite::Message};
use futures::prelude::*;
use rmp_serde;
use crate::{Authentication, LighthouseResult, Display, ClientMessage, Payload};

pub struct Connection {
    authentication: Authentication,
    connection: WebSocketStream<ConnectStream>,
    request_id: i32,
}

impl Connection {
    pub async fn new(authentication: Authentication) -> LighthouseResult<Self> {
        Ok(Self {
            authentication,
            connection: connect_async("wss://lighthouse.uni-kiel.de/websocket").await?.0,
            request_id: 0,
        })
    }

    pub async fn send_display(&mut self, display: Display) -> LighthouseResult<()> {
        let username = self.authentication.username.clone();
        self.send_message("PUT", ["user", username.as_str(), "model"], Payload::Display(display)).await
    }

    async fn send_message(&mut self, verb: &str, path: impl IntoIterator<Item=&str>, payload: Payload) -> LighthouseResult<()> {
        let request_id = self.request_id;
        self.request_id += 1;

        self.send(rmp_serde::to_vec(&ClientMessage {
            request_id,
            authentication: self.authentication.clone(),
            path: path.into_iter().map(|s| s.to_owned()).collect(),
            verb: verb.to_owned(),
            payload
        })?).await
    }

    async fn send(&mut self, bytes: impl Into<Vec<u8>>) -> LighthouseResult<()> {
        Ok(self.connection.send(Message::Binary(bytes.into())).await?)
    }
}
