use std::{collections::HashMap, sync::Arc};

use async_tungstenite::tungstenite::{Message, self};
use futures::{prelude::*, channel::mpsc::{Sender, self, Receiver}, stream::{SplitSink, SplitStream}, lock::Mutex};
use tracing::{warn, error};
use crate::{Authentication, Result, Frame, ClientMessage, Payload, Error, ServerMessage, Spawner};

/// A connection to the lighthouse server for sending requests and receiving events.
pub struct Lighthouse<S> {
    /// The sink-part of the WebSocket connection.
    ws_sink: SplitSink<S, Message>,
    /// The response/event handlers, keyed by request id.
    txs: Arc<Mutex<HashMap<i32, Sender<ServerMessage>>>>,
    /// The credentials used to authenticate with the lighthouse.
    authentication: Authentication,
    /// The next request id. Incremented on every request.
    request_id: i32,
}

impl<S> Lighthouse<S>
    where S: Stream<Item = tungstenite::Result<Message>>
           + Sink<Message, Error = tungstenite::Error>
           + Unpin
           + Send
           + 'static {
    /// Connects to the lighthouse using the given credentials.
    /// Asynchronously runs a receive loop using the provided spawner.
    pub fn new<W>(web_socket: S, authentication: Authentication) -> Result<Self> where W: Spawner {
        let (ws_sink, ws_stream) = web_socket.split();
        let txs = Arc::new(Mutex::new(HashMap::new()));
        let lh = Self {
            ws_sink,
            txs: txs.clone(),
            authentication,
            request_id: 0,
        };
        W::spawn(Self::run_receive_loop(ws_stream, txs));
        Ok(lh)
    }

    /// Runs a loop that continuously receives events.
    async fn run_receive_loop(mut ws_stream: SplitStream<S>, txs: Arc<Mutex<HashMap<i32, Sender<ServerMessage>>>>) {
        loop {
            match Self::receive_message_from(&mut ws_stream).await {
                Ok(msg) => {
                    let mut txs = txs.lock().await;
                    if let Some(request_id) = msg.request_id {
                        if let Some(tx) = txs.get_mut(&request_id) {
                            if let Err(e) = tx.send(msg).await {
                                warn!("Could not send received message: {:?}", e);
                            }
                        } else {
                            warn!("No channel registered for request id in received message: {:?}", msg);
                        }
                    } else {
                        warn!("Got message without request id from server: {:?}", msg);
                    }
                },
                Err(e) => error!("Bad message: {:?}", e),
            }
        }
    }

    /// Receives a ServerMessage from the lighthouse.
    async fn receive_message_from(ws_stream: &mut SplitStream<S>) -> Result<ServerMessage> {
        let bytes = Self::receive_raw_from(ws_stream).await?;
        let message = rmp_serde::from_slice(&bytes)?;
        Ok(message)
    }

    /// Receives raw bytes from the lighthouse via the WebSocket connection.
    async fn receive_raw_from(ws_stream: &mut SplitStream<S>) -> Result<Vec<u8>> {
        loop {
            let message = ws_stream.next().await.ok_or_else(|| Error::custom("Got no message"))??;
            match message {
                Message::Binary(bytes) => break Ok(bytes),
                // We ignore pings for now
                Message::Ping(_) => {},
                _ => warn!("Got non-binary message: {:?}", message),
            }
        }
    }

    /// Replaces the user's lighthouse model with the given frame.
    pub async fn put_model(&mut self, frame: Frame) -> Result<()> {
        let username = self.authentication.username.clone();
        self.put(["user", username.as_str(), "model"], Payload::Frame(frame)).await
    }

    /// Requests a stream of events (including key/controller events) for the user's lighthouse model.
    pub async fn stream_model(&mut self) -> Result<Receiver<ServerMessage>> {
        let username = self.authentication.username.clone();
        self.stream(["user", username.as_str(), "model"], Payload::Empty).await
    }

    /// Performs a PUT request to the given path with the given payload.
    pub async fn put(&mut self, path: impl IntoIterator<Item=&str>, payload: Payload) -> Result<()> {
        self.request("PUT", path, payload).await
    }

    /// Performs a single request to the given path with the given payload.
    pub async fn request(&mut self, verb: &str, path: impl IntoIterator<Item=&str>, payload: Payload) -> Result<()> {
        assert_ne!(verb, "STREAM", "Lighthouse::request may only be used for one-off requests, use Lighthouse::stream for streaming.");
        let request_id = self.send_request(verb, path, payload).await?;
        let response = self.receive_single(request_id).await?;
        response.check()?;
        Ok(())
    }
    
    /// Performs a STREAM request to the given path with the given payload.
    pub async fn stream(&mut self, path: impl IntoIterator<Item=&str>, payload: Payload) -> Result<Receiver<ServerMessage>> {
        let request_id = self.send_request("STREAM", path, payload).await?;
        let stream = self.receive_streaming(request_id).await?;
        Ok(stream)
    }

    /// Sends a request to the given path with the given payload.
    async fn send_request(&mut self, verb: &str, path: impl IntoIterator<Item=&str>, payload: Payload) -> Result<i32> {
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
        Ok(request_id)
    }

    /// Sends a generic message to the lighthouse.
    async fn send_message(&mut self, message: &ClientMessage) -> Result<()> {
        self.send_raw(rmp_serde::to_vec_named(message)?).await
    }

    /// Receives a single response for the given request id.
    async fn receive_single(&self, request_id: i32) -> Result<ServerMessage> {
        let mut txs = self.txs.lock().await;
        let (tx, mut rx) = mpsc::channel(1);
        txs.insert(request_id, tx);
        rx.next().await.ok_or_else(|| Error::Custom(format!("No response for {}", request_id)))
    }

    /// Receives a stream of responses for the given request id.
    async fn receive_streaming(&self, request_id: i32) -> Result<Receiver<ServerMessage>> {
        // TODO: Return a custom wrapper type (instead of a standard mpsc::Receiver)
        //       that keeps a reference to the `txs` + the request id and deregisters
        //       the corresponding sender on drop, along with sending a STOP
        //       request.

        let mut txs = self.txs.lock().await;
        let (tx, rx) = mpsc::channel(4);
        txs.insert(request_id, tx);
        Ok(rx)
    }

    /// Sends raw bytes to the lighthouse via the WebSocket connection.
    async fn send_raw(&mut self, bytes: impl Into<Vec<u8>>) -> Result<()> {
        Ok(self.ws_sink.send(Message::Binary(bytes.into())).await?)
    }
}
