use lighthouse_client::{Lighthouse, Result};
use lighthouse_protocol::{Authentication, Color, Frame};
use tracing::info;
use std::env;

async fn run(auth: Authentication) -> Result<()> {
    let mut lh = Lighthouse::connect_with_tokio(auth).await?;
    info!("Connected to the Lighthouse server");

    lh.put_model(Frame::fill(Color::BLACK)).await?;

    Ok(())
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    tracing_subscriber::fmt().init();
    _ = dotenvy::dotenv();

    let username = env::var("LIGHTHOUSE_USER").unwrap();
    let token = env::var("LIGHTHOUSE_TOKEN").unwrap();
    let auth = Authentication::new(username.as_str(), token.as_str());

    run(auth).await.unwrap();
}
