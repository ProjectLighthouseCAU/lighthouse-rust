use async_std::task;
use futures::Future;

use crate::Spawner;

/// A spawner that creates asynchronous tasks.
pub enum AsyncStdSpawner {}

impl Spawner for AsyncStdSpawner {
    fn spawn<F>(future: F) where F: Future + Send + 'static, F::Output: Send {
        task::spawn(future);
    }
}
