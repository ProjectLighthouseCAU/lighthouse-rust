use futures::StreamExt;
use lighthouse_client::{Lighthouse, Result, LIGHTHOUSE_URL};
use lighthouse_protocol::{Authentication, Payload};
use tracing::info;
use std::env;

async fn run(url: &str, auth: Authentication) -> Result<()> {
    let mut lh = Lighthouse::connect_with_tokio_to(url, auth).await?;
    info!("Connected to the Lighthouse server");

    // Stream input events
    let mut stream = lh.stream_model().await?;
    while let Some(msg) = stream.next().await {
        if let Payload::InputEvent(event) = msg.payload {
            info!("Got input event: {:?}", event)
        }
    }

    Ok(())
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    tracing_subscriber::fmt().init();
    _ = dotenvy::dotenv();

    let url = env::var("LIGHTHOUSE_URL").unwrap_or_else(|_| LIGHTHOUSE_URL.to_owned());
    let username = env::var("LIGHTHOUSE_USER").unwrap();
    let token = env::var("LIGHTHOUSE_TOKEN").unwrap();
    let auth = Authentication::new(username.as_str(), token.as_str());

    run(&url, auth).await.unwrap();
}
