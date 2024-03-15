#[cfg(feature = "async-std")]
mod async_std;
#[cfg(feature = "tokio")]
mod tokio;

use futures::Future;

#[cfg(feature = "async-std")]
pub use self::async_std::*;
#[cfg(feature = "tokio")]
pub use self::tokio::*;

/// A facility to spawn asynchronous tasks.
pub trait Spawner {
    fn spawn<F>(future: F) where F: Future + Send + 'static, F::Output: Send;
}
