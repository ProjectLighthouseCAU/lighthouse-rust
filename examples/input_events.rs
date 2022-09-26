use async_std::{task, stream::StreamExt};
use lighthouse_client::{Lighthouse, Authentication, Result, Payload};
use tracing::info;
use tracing_subscriber::EnvFilter;
use std::env;

async fn run(auth: Authentication) -> Result<()> {
    let mut lh = Lighthouse::connect_with_async_std(auth).await?;
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

fn main() {
    tracing_subscriber::fmt()
        .compact()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let username = env::var("LIGHTHOUSE_USER").unwrap();
    let token = env::var("LIGHTHOUSE_TOKEN").unwrap();
    let auth = Authentication::new(username.as_str(), token.as_str());

    task::block_on(run(auth)).unwrap();
}
