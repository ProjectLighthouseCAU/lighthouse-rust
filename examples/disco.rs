use async_std::task;
use lighthouse_client::{Lighthouse, Authentication, Result, Frame};
use tracing::info;
use tracing_subscriber::{prelude::*, EnvFilter};
use std::{env, time::Duration};

async fn run(auth: Authentication) -> Result<()> {
    let mut lh = Lighthouse::connect_with_async_std(auth).await?;
    info!("Connected to the Lighthouse server");

    loop {
        lh.put_model(Frame::fill(rand::random())).await?;
        info!("Sent frame");

        task::sleep(Duration::from_secs(1)).await;
    }
}

fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().with_filter(EnvFilter::from_default_env()))
        .init();

    let username = env::var("LIGHTHOUSE_USER").unwrap();
    let token = env::var("LIGHTHOUSE_TOKEN").unwrap();
    let auth = Authentication::new(username.as_str(), token.as_str());

    task::block_on(run(auth)).unwrap();
}
