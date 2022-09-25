use std::collections::{HashMap, VecDeque};

use async_tungstenite::tungstenite::{Message, self};
use futures::prelude::*;
use tracing::warn;
use rmp_serde;
use crate::{Authentication, Result, Frame, ClientMessage, Payload, Error, ServerMessage, InputEvent};

/// A connection to the lighthouse server for sending requests and receiving events.
pub struct Lighthouse<S> {
    web_socket: S,
    authentication: Authentication,
    queued_messages: VecDeque<ServerMessage>,
    request_id: i32,
}

impl<S> Lighthouse<S> {
    /// Connects to the lighthouse using the given credentials.
    pub fn new(web_socket: S, authentication: Authentication) -> Result<Self> {
        Ok(Self {
            web_socket,
            authentication,
            queued_messages: VecDeque::new(),
            request_id: 0,
        })
    }
}

impl<S> Lighthouse<S> where S: Stream<Item = tungstenite::Result<Message>> + Sink<Message, Error = tungstenite::Error> + Unpin {
    /// Replaces the user's lighthouse model with the given frame.
    pub async fn put_frame(&mut self, frame: Frame) -> Result<()> {
        let username = self.authentication.username.clone();
        self.send_request("PUT", ["user", username.as_str(), "model"], Payload::Frame(frame)).await
    }

    /// Requests a stream of events (including key/controller events) for the user's lighthouse model.
    pub async fn request_stream(&mut self) -> Result<()> {
        let username = self.authentication.username.clone();
        self.send_request("STREAM", ["user", username.as_str(), "model"], Payload::Empty).await
    }

    /// Sends a request to the given path with the given payload.
    pub async fn send_request(&mut self, verb: &str, path: impl IntoIterator<Item=&str>, payload: Payload) -> Result<()> {
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
    pub async fn send_message(&mut self, message: &ClientMessage) -> Result<()> {
        self.send(rmp_serde::to_vec_named(message)?).await
    }

    /// Receives the response to a message.
    async fn check_response(&mut self, request_id: i32) -> Result<()> {
        let response = self.receive_message_where(|m| m.request_id == Some(request_id)).await?;
        if response.code == 200 {
            Ok(())
        } else {
            Err(Error::Server { code: response.code, message: response.response })
        }
    }

    /// Receives the next input event from the lighthouse.
    pub async fn receive_input_event(&mut self) -> Result<InputEvent> {
        self.receive_message_filtering(|m| match m.payload {
            Payload::InputEvent(event) => Some(event),
            _ => None
        }).await
    }

    /// Receives the next (generic) message from the lighthouse.
    pub async fn receive_message(&mut self) -> Result<ServerMessage> {
        self.receive_message_filtering(|m| Some(m.clone())).await
    }

    /// Receives the next (generic) message that satisfies the given predicate from the lighthouse.
    pub async fn receive_message_where(&mut self, filter: impl Fn(&ServerMessage) -> bool) -> Result<ServerMessage> {
        self.receive_message_filtering(|m| if filter(&m) { Some(m.clone()) } else { None }).await
    }

    /// Receives the next (generic) message using the given filter-mapper from the lighthouse.
    pub async fn receive_message_filtering<T>(&mut self, filter: impl Fn(&ServerMessage) -> Option<T>) -> Result<T> {
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
    async fn send(&mut self, bytes: impl Into<Vec<u8>>) -> Result<()> {
        Ok(self.web_socket.send(Message::Binary(bytes.into())).await?)
    }

    /// Receives raw bytes from the lighthouse via the WebSocket connection.
    async fn receive(&mut self) -> Result<Vec<u8>> {
        loop {
            let message = self.web_socket.next().await.ok_or_else(|| Error::custom("Got no message"))??;
            match message {
                Message::Binary(bytes) => break Ok(bytes),
                // We ignore pings for now
                Message::Ping(_) => {},
                _ => warn!("Got non-binary message: {:?}", message),
            }
        }
    }
}
