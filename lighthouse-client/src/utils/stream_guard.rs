use std::{pin::Pin, task::{Context, Poll}};

use futures::Stream;
use pin_project::{pin_project, pinned_drop};

/// A stream wrapper that automatically runs a closure when dropped.
#[pin_project(PinnedDrop)]
pub struct StreamGuard<S, F> where S: Stream + Unpin, F: FnOnce() {
    #[pin]
    stream: S,
    on_drop: Option<F>,
}

impl<S, F> StreamGuard<S, F> where S: Stream + Unpin, F: FnOnce() {
    pub fn new(stream: S, on_drop: F) -> Self {
        Self { stream, on_drop: Some(on_drop) }
    }
}

impl<S, F> Stream for StreamGuard<S, F> where S: Stream + Unpin, F: FnOnce() {
    type Item = S::Item;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        Pin::new(&mut self.stream).poll_next(cx)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.stream.size_hint()
    }
}

#[pinned_drop]
impl<S, F> PinnedDrop for StreamGuard<S, F> where S: Stream + Unpin, F: FnOnce() {
    fn drop(mut self: Pin<&mut Self>) {
        self.on_drop.take().expect("No on_drop function in StreamGuard, was drop called twice or constructed wrongly?")()
    }
}

pub trait StreamExt: Stream + Sized + Unpin {
    fn guard<F>(self, on_drop: F) -> StreamGuard<Self, F> where F: FnOnce();
}

impl<S> StreamExt for S where S: Stream + Sized + Unpin {
    fn guard<F>(self, on_drop: F) -> StreamGuard<Self, F> where F: FnOnce() {
        StreamGuard::new(self, on_drop)
    }
}
