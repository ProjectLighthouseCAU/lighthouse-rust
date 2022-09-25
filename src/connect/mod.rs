#[cfg(feature = "async-std")]
pub mod async_std;
#[cfg(feature = "tokio")]
pub mod tokio;

// Convenience extensions for when only a single runtime is selected.
// The reason why don't simply define an extension trait (which the user can choose)
// for each runtime is that async traits aren't natively supported yet.

#[cfg(all(feature = "async-std", not(feature = "tokio")))]
impl crate::Lighthouse<async_tungstenite::WebSocketStream<async_tungstenite::async_std::ConnectStream>> {
    /// Connects to the lighthouse server at the given URL.
    pub async fn connect_to(url: &str, authentication: crate::Authentication) -> crate::LighthouseResult<Self> {
        self::async_std::connect_to(url, authentication).await
    }

    /// Connects to the lighthouse server at the default URL.
    pub async fn connect(authentication: crate::Authentication) -> crate::LighthouseResult<Self> {
        self::async_std::connect(authentication).await
    }
}

#[cfg(all(feature = "tokio", not(feature = "async-std")))]
impl crate::Lighthouse<async_tungstenite::WebSocketStream<async_tungstenite::tokio::ConnectStream>> {
    /// Connects to the lighthouse server at the given URL.
    pub async fn connect_to(url: &str, authentication: crate::Authentication) -> crate::LighthouseResult<Self> {
        self::tokio::connect_to(url, authentication).await
    }

    /// Connects to the lighthouse server at the default URL.
    pub async fn connect(authentication: crate::Authentication) -> crate::LighthouseResult<Self> {
        self::tokio::connect(authentication).await
    }
}
