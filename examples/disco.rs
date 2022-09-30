use lighthouse_client::{Lighthouse, Authentication, Result, Frame};
use tracing::info;
use tokio::time;
use std::{env, time::Duration};

async fn run(auth: Authentication) -> Result<()> {
    let mut lh = Lighthouse::connect_with_tokio(auth).await?;
    info!("Connected to the Lighthouse server");

    loop {
        lh.put_model(Frame::fill(rand::random())).await?;
        info!("Sent frame");

        time::sleep(Duration::from_secs(1)).await;
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    tracing_subscriber::fmt().init();

    let username = env::var("LIGHTHOUSE_USER").unwrap();
    let token = env::var("LIGHTHOUSE_TOKEN").unwrap();
    let auth = Authentication::new(username.as_str(), token.as_str());

    run(auth).await.unwrap();
}
