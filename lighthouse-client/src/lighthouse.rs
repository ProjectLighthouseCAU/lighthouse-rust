use std::{collections::HashMap, sync::Arc, fmt::Debug};

use async_tungstenite::tungstenite::{Message, self};
use futures::{prelude::*, channel::mpsc::{Sender, self}, stream::{SplitSink, SplitStream}, lock::Mutex};
use lighthouse_protocol::{Authentication, ClientMessage, DirectoryTree, Frame, LaserMetrics, Model, ServerMessage, Value, Verb};
use serde::{Deserialize, Serialize};
use tracing::{warn, error, debug, info};
use crate::{Check, Error, Result, Spawner};

/// A connection to the lighthouse server for sending requests and receiving events.
pub struct Lighthouse<S> {
    /// The sink-part of the WebSocket connection.
    ws_sink: SplitSink<S, Message>,
    /// The response/event handlers, keyed by request id.
    txs: Arc<Mutex<HashMap<i32, Sender<ServerMessage<Value>>>>>,
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
    #[tracing::instrument(skip(ws_stream, txs))]
    async fn run_receive_loop(mut ws_stream: SplitStream<S>, txs: Arc<Mutex<HashMap<i32, Sender<ServerMessage<Value>>>>>) {
        loop {
            match Self::receive_message_from(&mut ws_stream).await {
                Ok(msg) => {
                    let mut txs = txs.lock().await;
                    if let Some(request_id) = msg.request_id {
                        if let Some(tx) = txs.get_mut(&request_id) {
                            if let Err(e) = tx.send(msg).await {
                                if e.is_disconnected() {
                                    info!("Receiver for request id {} disconnected, removing the sender...", request_id);
                                    txs.remove(&request_id);
                                } else {
                                    warn!("Could not send message for request id {} via channel: {:?}", request_id, e);
                                }
                            }
                        } else {
                            warn!("No channel registered for request id {}", request_id);
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
    #[tracing::instrument(skip(ws_stream))]
    async fn receive_message_from<P>(ws_stream: &mut SplitStream<S>) -> Result<ServerMessage<P>>
    where
        P: for<'de> Deserialize<'de> {
        let bytes = Self::receive_raw_from(ws_stream).await?;
        let message = rmp_serde::from_slice(&bytes)?;
        Ok(message)
    }

    /// Receives raw bytes from the lighthouse via the WebSocket connection.
    #[tracing::instrument(skip(ws_stream))]
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
    pub async fn put_model(&mut self, frame: Frame) -> Result<ServerMessage<()>> {
        let username = self.authentication.username.clone();
        self.put(&["user", username.as_str(), "model"], Model::Frame(frame)).await
    }

    /// Requests a stream of events (including key/controller events) for the user's lighthouse model.
    pub async fn stream_model(&mut self) -> Result<impl Stream<Item = Result<ServerMessage<Model>>>> {
        let username = self.authentication.username.clone();
        self.stream(&["user", username.as_str(), "model"], ()).await
    }

    /// Fetches lamp server metrics.
    pub async fn get_laser_metrics(&mut self) -> Result<ServerMessage<LaserMetrics>> {
        self.get(&["metrics", "laser"]).await
    }

    /// Combines PUT and CREATE. Requires CREATE and WRITE permission.
    pub async fn post<P>(&mut self, path: &[&str], payload: P) -> Result<ServerMessage<()>>
    where
        P: Serialize {
        self.perform(&Verb::Post, path, payload).await
    }

    /// Updates the resource at the given path with the given payload. Requires WRITE permission.
    pub async fn put<P>(&mut self, path: &[&str], payload: P) -> Result<ServerMessage<()>>
    where
        P: Serialize {
        self.perform(&Verb::Put, path, payload).await
    }

    /// Creates a resource at the given path. Requires CREATE permission.
    pub async fn create(&mut self, path: &[&str]) -> Result<ServerMessage<()>> {
        self.perform(&Verb::Create, path, ()).await
    }

    /// Deletes a resource at the given path. Requires DELETE permission.
    pub async fn delete(&mut self, path: &[&str]) -> Result<ServerMessage<()>> {
        self.perform(&Verb::Delete, path, ()).await
    }

    /// Creates a directory at the given path. Requires CREATE permission.
    pub async fn mkdir(&mut self, path: &[&str]) -> Result<ServerMessage<()>> {
        self.perform(&Verb::Mkdir, path, ()).await
    }

    /// Lists the directory tree at the given path. Requires READ permission.
    pub async fn list(&mut self, path: &[&str]) -> Result<ServerMessage<DirectoryTree>> {
        self.perform(&Verb::List, path, ()).await
    }

    /// Gets the resource at the given path. Requires READ permission.
    pub async fn get<R>(&mut self, path: &[&str]) -> Result<ServerMessage<R>>
    where
        R: for<'de> Deserialize<'de> {
        self.perform(&Verb::Get, path, ()).await
    }

    /// Links the given source to the given destination path.
    pub async fn link(&mut self, src_path: &[&str], dest_path: &[&str]) -> Result<ServerMessage<()>> {
        self.perform(&Verb::Link, dest_path, src_path).await
    }

    /// Unlinks the given source from the given destination path.
    pub async fn unlink(&mut self, src_path: &[&str], dest_path: &[&str]) -> Result<ServerMessage<()>> {
        self.perform(&Verb::Unlink, dest_path, src_path).await
    }

    /// Performs a single request to the given path with the given payload.
    #[tracing::instrument(skip(self, payload))]
    pub async fn perform<P, R>(&mut self, verb: &Verb, path: &[&str], payload: P) -> Result<ServerMessage<R>>
    where
        P: Serialize,
        R: for<'de> Deserialize<'de> {
        assert_ne!(verb, &Verb::Stream, "Lighthouse::perform may only be used for one-off requests, use Lighthouse::stream for streaming.");
        let request_id = self.send_request(verb, path, payload).await?;
        let response = self.receive_single(request_id).await?.check()?.decode_payload()?;
        Ok(response)
    }
    
    /// Performs a STREAM request to the given path with the given payload.
    #[tracing::instrument(skip(self, payload))]
    pub async fn stream<P, R>(&mut self, path: &[&str], payload: P) -> Result<impl Stream<Item = Result<ServerMessage<R>>>>
    where
        P: Serialize,
        R: for<'de> Deserialize<'de> {
        let request_id = self.send_request(&Verb::Stream, path, payload).await?;
        let stream = self.receive_streaming(request_id).await?;
        Ok(stream)
    }

    /// Sends a request to the given path with the given payload.
    async fn send_request<P>(&mut self, verb: &Verb, path: &[&str], payload: P) -> Result<i32>
    where
        P: Serialize {
        let path = path.into_iter().map(|s| s.to_string()).collect();
        let request_id = self.request_id;
        debug! { %request_id, "Sending request" };
        self.request_id += 1;
        self.send_message(&ClientMessage {
            request_id,
            authentication: self.authentication.clone(),
            path,
            meta: HashMap::new(),
            verb: verb.clone(),
            payload
        }).await?;
        Ok(request_id)
    }

    /// Sends a generic message to the lighthouse.
    async fn send_message<P>(&mut self, message: &ClientMessage<P>) -> Result<()>
    where
        P: Serialize {
        self.send_raw(rmp_serde::to_vec_named(message)?).await
    }

    /// Receives a single response for the given request id.
    #[tracing::instrument(skip(self))]
    async fn receive_single<R>(&self, request_id: i32) -> Result<ServerMessage<R>>
    where
        R: for<'de> Deserialize<'de> {
        let mut rx = self.receive(request_id).await?;
        rx.next().await.ok_or_else(|| Error::Custom(format!("No response for {}", request_id)))?
    }

    /// Receives a stream of responses for the given request id.
    #[tracing::instrument(skip(self))]
    async fn receive_streaming<R>(&self, request_id: i32) -> Result<impl Stream<Item = Result<ServerMessage<R>>>>
    where
        R: for<'de> Deserialize<'de> {
        // TODO: Return a custom wrapper type (instead of a standard mpsc::Receiver)
        //       that keeps a reference to the `txs` + the request id and deregisters
        //       the corresponding sender on drop, along with sending a STOP
        //       request.

        self.receive(request_id).await
    }

    async fn receive<R>(&self, request_id: i32) -> Result<impl Stream<Item = Result<ServerMessage<R>>>>
    where
        R: for<'de> Deserialize<'de> {
        let mut txs = self.txs.lock().await;
        let (tx, rx) = mpsc::channel(4);
        txs.insert(request_id, tx);
        Ok(rx.map(|s| Ok(s.decode_payload()?)))
    }

    /// Sends raw bytes to the lighthouse via the WebSocket connection.
    async fn send_raw(&mut self, bytes: impl Into<Vec<u8>> + Debug) -> Result<()> {
        Ok(self.ws_sink.send(Message::Binary(bytes.into())).await?)
    }

    /// Fetches the credentials used to authenticate with the lighthouse.
    pub fn authentication(&self) -> &Authentication {
        &self.authentication
    }
}
