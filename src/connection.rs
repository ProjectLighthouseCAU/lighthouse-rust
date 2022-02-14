use std::collections::HashMap;

use async_tungstenite::{async_std::{connect_async, ConnectStream}, WebSocketStream, tungstenite::Message};
use futures::prelude::*;
use log::warn;
use rmp_serde;
use crate::{Authentication, LighthouseResult, Display, ClientMessage, Payload, LighthouseError, ServerMessage};

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
        self.send_request("PUT", ["user", username.as_str(), "model"], Payload::Display(display)).await
    }

    pub async fn send_request(&mut self, verb: &str, path: impl IntoIterator<Item=&str>, payload: Payload) -> LighthouseResult<()> {
        let request_id = self.request_id;
        self.request_id += 1;
        self.send_message(&ClientMessage {
            request_id,
            authentication: self.authentication.clone(),
            path: path.into_iter().map(|s| s.to_owned()).collect(),
            meta: HashMap::new(),
            verb: verb.to_owned(),
            payload
        }).await?;
        self.check_response().await
    }

    pub async fn send_message(&mut self, message: &ClientMessage) -> LighthouseResult<()> {
        self.send(rmp_serde::to_vec_named(message)?).await
    }

    async fn check_response(&mut self) -> LighthouseResult<()> {
        let response = self.receive_message().await?;
        if response.code == 200 {
            Ok(())
        } else {
            Err(LighthouseError::Server { code: response.code, message: response.response })
        }
    }

    pub async fn receive_message(&mut self) -> LighthouseResult<ServerMessage> {
        let bytes = self.receive().await?;
        Ok(rmp_serde::from_slice(&bytes)?)
    }

    async fn send(&mut self, bytes: impl Into<Vec<u8>>) -> LighthouseResult<()> {
        Ok(self.connection.send(Message::Binary(bytes.into())).await?)
    }

    async fn receive(&mut self) -> LighthouseResult<Vec<u8>> {
        loop {
            let message = self.connection.next().await.ok_or_else(|| LighthouseError::custom("Got no message"))??;
            match message {
                Message::Binary(bytes) => break Ok(bytes),
                // We ignore pings for now
                Message::Ping(_) => {},
                _ => warn!("Got non-binary message: {:?}", message),
            }
        }
    }
}
