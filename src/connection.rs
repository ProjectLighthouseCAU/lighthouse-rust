use std::collections::{HashMap, VecDeque};

use async_tungstenite::{async_std::{connect_async, ConnectStream}, WebSocketStream, tungstenite::Message};
use futures::prelude::*;
use log::warn;
use rmp_serde;
use crate::{Authentication, LighthouseResult, Display, ClientMessage, Payload, LighthouseError, ServerMessage, InputEvent};

/// A connection to the lighthouse server for sending requests and receiving events.
pub struct Connection {
    authentication: Authentication,
    connection: WebSocketStream<ConnectStream>,
    queued_messages: VecDeque<ServerMessage>,
    request_id: i32,
}

impl Connection {
    /// Connects to the lighthouse using the given credentials.
    pub async fn new(authentication: Authentication) -> LighthouseResult<Self> {
        Ok(Self {
            authentication,
            connection: connect_async("wss://lighthouse.uni-kiel.de/websocket").await?.0,
            queued_messages: VecDeque::new(),
            request_id: 0,
        })
    }

    /// Sends a display (frame) to the user's lighthouse.
    pub async fn send_display(&mut self, display: Display) -> LighthouseResult<()> {
        let username = self.authentication.username.clone();
        self.send_request("PUT", ["user", username.as_str(), "model"], Payload::Display(display)).await
    }

    /// Requests a stream of events (including key/controller events) for the user's lighthouse.
    pub async fn request_stream(&mut self) -> LighthouseResult<()> {
        let username = self.authentication.username.clone();
        self.send_request("STREAM", ["user", username.as_str(), "model"], Payload::Empty).await
    }

    /// Sends a request to the given path with the given payload.
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
        self.check_response(request_id).await
    }

    /// Sends a generic message to the lighthouse.
    pub async fn send_message(&mut self, message: &ClientMessage) -> LighthouseResult<()> {
        self.send(rmp_serde::to_vec_named(message)?).await
    }

    /// Receives the response to a message.
    async fn check_response(&mut self, request_id: i32) -> LighthouseResult<()> {
        let response = self.receive_message_where(|m| m.request_id == Some(request_id)).await?;
        if response.code == 200 {
            Ok(())
        } else {
            Err(LighthouseError::Server { code: response.code, message: response.response })
        }
    }

    /// Receives the next input event from the lighthouse.
    pub async fn receive_input_event(&mut self) -> LighthouseResult<InputEvent> {
        self.receive_message_filtering(|m| match m.payload {
            Payload::InputEvent(event) => Some(event),
            _ => None
        }).await
    }

    /// Receives the next (generic) message from the lighthouse.
    pub async fn receive_message(&mut self) -> LighthouseResult<ServerMessage> {
        self.receive_message_filtering(|m| Some(m.clone())).await
    }

    /// Receives the next (generic) message that satisfies the given predicate from the lighthouse.
    pub async fn receive_message_where(&mut self, filter: impl Fn(&ServerMessage) -> bool) -> LighthouseResult<ServerMessage> {
        self.receive_message_filtering(|m| if filter(&m) { Some(m.clone()) } else { None }).await
    }

    /// Receives the next (generic) message using the given filter-mapper from the lighthouse.
    pub async fn receive_message_filtering<T>(&mut self, filter: impl Fn(&ServerMessage) -> Option<T>) -> LighthouseResult<T> {
        // Try to find the message in the queue
        for _ in 0..self.queued_messages.len() {
            if let Some(message) = self.queued_messages.pop_front() {
                if let Some(value) = filter(&message) {
                    return Ok(value);
                } else {
                    self.queued_messages.push_back(message);
                }
            }
        }

        loop {
            // Otherwise receive the next message
            let bytes = self.receive().await?;
            let message = rmp_serde::from_slice(&bytes)?;
            if let Some(value) = filter(&message) {
                return Ok(value);
            } else {
                self.queued_messages.push_back(message);
            }
        }
    }

    /// Sends raw bytes to the lighthouse via the WebSocket connection.
    async fn send(&mut self, bytes: impl Into<Vec<u8>>) -> LighthouseResult<()> {
        Ok(self.connection.send(Message::Binary(bytes.into())).await?)
    }

    /// Receives raw bytes from the lighthouse via the WebSocket connection.
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
