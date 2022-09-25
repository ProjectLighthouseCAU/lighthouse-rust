use futures::Future;
use tokio::task;

use crate::Spawner;

/// A spawner that creates asynchronous tasks.
pub enum TokioSpawner {}

impl Spawner for TokioSpawner {
    fn spawn<F>(future: F) where F: Future + Send + 'static, F::Output: Send {
        task::spawn(future);
    }
}
