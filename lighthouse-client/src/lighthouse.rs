use std::{collections::HashMap, fmt::Debug, sync::{atomic::{AtomicI32, Ordering}, Arc}};

use async_tungstenite::tungstenite::{Message, self};
use futures::{prelude::*, channel::mpsc::{Sender, self}, stream::{SplitSink, SplitStream}, lock::Mutex};
use lighthouse_protocol::{Authentication, ClientMessage, DirectoryTree, Frame, InputEvent, LaserMetrics, Model, ServerMessage, Value, Verb};
use serde::{Deserialize, Serialize};
use stream_guard::GuardStreamExt;
use tracing::{warn, error, debug, info};
use crate::{Check, Error, Result, Spawner};

/// A connection to the lighthouse server for sending requests and receiving events.
pub struct Lighthouse<S> {
    /// The sink-part of the WebSocket connection.
    ws_sink: Arc<Mutex<SplitSink<S, Message>>>,
    /// The response/event slots, keyed by request id.
    slots: Arc<Mutex<HashMap<i32, Slot<ServerMessage<Value>>>>>,
    /// The credentials used to authenticate with the lighthouse.
    authentication: Authentication,
    /// The next request id. Incremented on every request.
    request_id: Arc<AtomicI32>,
}

/// A facility for coordinating asynchronous responses to a request between a
/// requesting task and a receive loop task.
enum Slot<M> {
    /// Indicates that messages were received before the requesting task
    /// registered the slot. **The receive loop** will construct this variant in
    /// that case, i.e. store the already received messages in a
    /// [`Slot::EarlyMessages`].
    EarlyMessages(Vec<M>),
    /// Indicates that no messages were received before the requesting task
    /// registered the slot. **The requesting thread** will construct this
    /// variant in that case, i.e. create a channel, store the sender in a
    /// [`Slot::WaitForMessages`] for the receive loop and then return the
    /// receiver.
    WaitForMessages(Sender<M>),
}

impl<S> Lighthouse<S>
    where S: Stream<Item = tungstenite::Result<Message>>
           + Sink<Message, Error = tungstenite::Error>
           + Send
           + 'static {
    /// Connects to the lighthouse using the given credentials.
    /// Asynchronously runs a receive loop using the provided spawner.
    pub fn new<W>(web_socket: S, authentication: Authentication) -> Result<Self> where W: Spawner {
        let (ws_sink, ws_stream) = web_socket.split();
        let slots = Arc::new(Mutex::new(HashMap::new()));
        let lh = Self {
            ws_sink: Arc::new(Mutex::new(ws_sink)),
            slots: slots.clone(),
            authentication,
            request_id: Arc::new(AtomicI32::new(0)),
        };
        W::spawn(Self::run_receive_loop(ws_stream, slots));
        Ok(lh)
    }

    /// Runs a loop that continuously receives events.
    #[tracing::instrument(skip(ws_stream, slots))]
    async fn run_receive_loop(mut ws_stream: SplitStream<S>, slots: Arc<Mutex<HashMap<i32, Slot<ServerMessage<Value>>>>>) {
        loop {
            match Self::receive_message_from(&mut ws_stream).await {
                Ok(msg) => {
                    let mut slots = slots.lock().await;
                    if let Some(request_id) = msg.request_id {
                        if let Some(slot) = slots.get_mut(&request_id) {
                            match slot {
                                Slot::EarlyMessages(msgs) => msgs.push(msg),
                                Slot::WaitForMessages(tx) => {
                                    if let Err(e) = tx.send(msg).await {
                                        if e.is_disconnected() {
                                            info!("Receiver for request id {} disconnected, removing the sender...", request_id);
                                            slots.remove(&request_id);
                                        } else {
                                            warn!("Could not send message for request id {} via channel: {:?}", request_id, e);
                                        }
                                    }
                                }
                            }
                        } else {
                            slots.insert(request_id, Slot::EarlyMessages(vec![msg]));
                        }
                    } else {
                        warn!("Got message without request id from server: {:?}", msg);
                    }
                },
                Err(Error::NoNextMessage) => {
                    info!("No next message available, closing receive loop");
                    break
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
            let message = ws_stream.next().await.ok_or_else(|| Error::NoNextMessage)??;
            match message {
                Message::Binary(bytes) => break Ok(bytes),
                Message::Ping(_) => {}, // Ignore pings for now
                Message::Close(_) => break Err(Error::ConnectionClosed),
                _ => warn!("Got non-binary message: {:?}", message),
            }
        }
    }

    /// Replaces the user's lighthouse model with the given frame.
    pub async fn put_model(&self, frame: Frame) -> Result<ServerMessage<()>> {
        let username = self.authentication.username.clone();
        self.put(&["user".into(), username, "model".into()], Model::Frame(frame)).await
    }

    /// Requests a stream of events (including key/controller events) for the user's lighthouse model.
    pub async fn stream_model(&self) -> Result<impl Stream<Item = Result<ServerMessage<Model>>>> {
        let username = self.authentication.username.clone();
        self.stream(&["user".into(), username, "model".into()], ()).await
    }

    /// Sends an input event to the user's input endpoint.
    /// 
    /// Note that this is the new API which not all clients may support.
    pub async fn put_input(&self, payload: InputEvent) -> Result<ServerMessage<()>> {
        let username = self.authentication.username.clone();
        self.put(&["user".into(), username, "input".into()], payload).await
    }

    /// Streams input events from the user's input endpoint.
    /// 
    /// Note that this is the new API which not all clients may support (in LUNA
    /// disabling the legacy mode will send events to this endpoint).  If your
    /// client or library does not support this, you may need to `stream_model`
    /// and parse `LegacyInputEvent`s from there.
    pub async fn stream_input(&self) -> Result<impl Stream<Item = Result<ServerMessage<InputEvent>>>> {
        let username = self.authentication.username.clone();
        Ok(
            self.stream(&["user".into(), username, "input".into()], ()).await?
                .skip(1) // Skip the persisted input (TODO: Should we handle this at the server level via some form of passthrough resources?)
        )
    }

    /// Fetches lamp server metrics.
    pub async fn get_laser_metrics(&self) -> Result<ServerMessage<LaserMetrics>> {
        self.get(&["metrics", "laser"]).await
    }

    /// Combines PUT and CREATE. Requires CREATE and WRITE permission.
    pub async fn post<P>(&self, path: &[impl AsRef<str> + Debug], payload: P) -> Result<ServerMessage<()>>
    where
        P: Serialize {
        self.perform(&Verb::Post, path, payload).await
    }

    /// Updates the resource at the given path with the given payload. Requires WRITE permission.
    pub async fn put<P>(&self, path: &[impl AsRef<str> + Debug], payload: P) -> Result<ServerMessage<()>>
    where
        P: Serialize {
        self.perform(&Verb::Put, path, payload).await
    }

    /// Creates a resource at the given path. Requires CREATE permission.
    pub async fn create(&self, path: &[impl AsRef<str> + Debug]) -> Result<ServerMessage<()>> {
        self.perform(&Verb::Create, path, ()).await
    }

    /// Deletes a resource at the given path. Requires DELETE permission.
    pub async fn delete(&self, path: &[impl AsRef<str> + Debug]) -> Result<ServerMessage<()>> {
        self.perform(&Verb::Delete, path, ()).await
    }

    /// Creates a directory at the given path. Requires CREATE permission.
    pub async fn mkdir(&self, path: &[impl AsRef<str> + Debug]) -> Result<ServerMessage<()>> {
        self.perform(&Verb::Mkdir, path, ()).await
    }

    /// Lists the directory tree at the given path. Requires READ permission.
    pub async fn list(&self, path: &[impl AsRef<str> + Debug]) -> Result<ServerMessage<DirectoryTree>> {
        self.perform(&Verb::List, path, ()).await
    }

    /// Gets the resource at the given path. Requires READ permission.
    pub async fn get<R>(&self, path: &[impl AsRef<str> + Debug]) -> Result<ServerMessage<R>>
    where
        R: for<'de> Deserialize<'de> {
        self.perform(&Verb::Get, path, ()).await
    }

    /// Links the given source to the given destination path.
    pub async fn link(&self, src_path: &[impl AsRef<str> + Debug], dest_path: &[impl AsRef<str> + Debug]) -> Result<ServerMessage<()>> {
        self.perform(&Verb::Link, dest_path, src_path.iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>()).await
    }

    /// Unlinks the given source from the given destination path.
    pub async fn unlink(&self, src_path: &[impl AsRef<str> + Debug], dest_path: &[impl AsRef<str> + Debug]) -> Result<ServerMessage<()>> {
        self.perform(&Verb::Unlink, dest_path, src_path.iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>()).await
    }

    /// Stops the given stream. **Should generally not be called manually**,
    /// since streams will automatically be stopped once dropped.
    pub async fn stop(&self, request_id: i32, path: &[impl AsRef<str> + Debug]) -> Result<ServerMessage<()>> {
        self.perform_with_id(request_id, &Verb::Stop, path, ()).await
    }

    /// Performs a single request to the given path with the given payload.
    #[tracing::instrument(skip(self, payload))]
    pub async fn perform<P, R>(&self, verb: &Verb, path: &[impl AsRef<str> + Debug], payload: P) -> Result<ServerMessage<R>>
    where
        P: Serialize,
        R: for<'de> Deserialize<'de> {
        let request_id = self.next_request_id();
        self.perform_with_id(request_id, verb, path, payload).await
    }

    /// Performs a single request to the given path with the given request id.
    #[tracing::instrument(skip(self, payload))]
    async fn perform_with_id<P, R>(&self, request_id: i32, verb: &Verb, path: &[impl AsRef<str> + Debug], payload: P) -> Result<ServerMessage<R>>
    where
        P: Serialize,
        R: for<'de> Deserialize<'de> {
        assert_ne!(verb, &Verb::Stream, "Lighthouse::perform may only be used for one-off requests, use Lighthouse::stream for streaming.");
        self.send_request(request_id, verb, path, payload).await?;
        let response = self.receive_single(request_id).await?.check()?.decode_payload()?;
        Ok(response)
    }
    
    /// Performs a STREAM request to the given path with the given payload.
    /// Automatically sends a STOP once dropped.
    #[tracing::instrument(skip(self, payload))]
    pub async fn stream<P, R>(&self, path: &[impl AsRef<str> + Debug], payload: P) -> Result<impl Stream<Item = Result<ServerMessage<R>>>>
    where
        P: Serialize,
        R: for<'de> Deserialize<'de> {
        let request_id = self.next_request_id();
        let path: Vec<String> = path.into_iter().map(|s| s.as_ref().to_string()).collect();
        self.send_request(request_id, &Verb::Stream, &path, payload).await?;
        let stream = self.receive_streaming(request_id).await?;
        Ok(stream.map(|m| Ok(m?.check()?.decode_payload()?)).guard({
            // Stop the stream on drop
            let this = (*self).clone();
            move || {
                tokio::spawn(async move {
                    if let Err(error) = this.stop(request_id, &path).await {
                        error! { ?path, %error, "Could not STOP stream" };
                    }
                });
            }
        }))
    }

    /// Sends a request to the given path with the given payload.
    async fn send_request<P>(&self, request_id: i32, verb: &Verb, path: &[impl AsRef<str> + Debug], payload: P) -> Result<i32>
    where
        P: Serialize {
        let path = path.into_iter().map(|s| s.as_ref().to_string()).collect();
        debug! { %request_id, "Sending request" };
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
    async fn send_message<P>(&self, message: &ClientMessage<P>) -> Result<()>
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
        self.receive(request_id).await
    }

    async fn receive<R>(&self, request_id: i32) -> Result<impl Stream<Item = Result<ServerMessage<R>>>>
    where
        R: for<'de> Deserialize<'de> {
        let rx = {
            let capacity = 4;
            let (tx, rx) = {
                let mut slots = self.slots.lock().await;
                if let Some(Slot::EarlyMessages(msgs)) = slots.get_mut(&request_id) {
                    let (mut tx, rx) = mpsc::channel(capacity.min(msgs.len()));
                    for msg in msgs.drain(..) {
                        tx.feed(msg).await.map_err(|e| Error::Custom(format!("Could not feed tx with early message: {}", e)))?;
                    } 
                    tx.flush().await.map_err(|e| Error::Custom(format!("Could not flush tx with early messages: {}", e)))?;
                    (tx, rx)
                } else {
                    mpsc::channel(capacity)
                }
            };
            self.slots.lock().await.insert(request_id, Slot::WaitForMessages(tx));
            rx
        };
        Ok(rx.map(|s| Ok(s.decode_payload()?)).guard({
            let slots = self.slots.clone();
            move || {
                tokio::spawn(async move {
                    let mut slots = slots.lock().await;
                    slots.remove(&request_id);
                });
            }
        }))
    }

    /// Sends raw bytes to the lighthouse via the WebSocket connection.
    async fn send_raw(&self, bytes: impl Into<Vec<u8>> + Debug) -> Result<()> {
        Ok(self.ws_sink.lock().await.send(Message::Binary(bytes.into())).await?)
    }

    /// Fetches the next request id.
    fn next_request_id(&self) -> i32 {
        self.request_id.fetch_add(1, Ordering::Relaxed)
    }

    /// Fetches the credentials used to authenticate with the lighthouse.
    pub fn authentication(&self) -> &Authentication {
        &self.authentication
    }

    /// Closes the WebSocket connection gracefully with a close message. While
    /// the server will usually also handle abruptly closed connections
    /// properly, it is recommended to always close the [``Lighthouse``].
    pub async fn close(&self) -> Result<()> {
        Ok(self.ws_sink.lock().await.close().await?)
    }
}

// For some reason `#[derive(Clone)]` adds the trait bound `S: Clone`, despite
// not actually being needed since the WebSocket sink is already wrapped in an
// `Arc`, therefore we implement `Clone` manually.

impl<S> Clone for Lighthouse<S> {
    fn clone(&self) -> Self {
        Self {
            ws_sink: self.ws_sink.clone(),
            slots: self.slots.clone(),
            authentication: self.authentication.clone(),
            request_id: self.request_id.clone(),
        }
    }
}
