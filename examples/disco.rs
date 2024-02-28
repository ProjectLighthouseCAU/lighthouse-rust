use lighthouse_client::{Authentication, Frame, Lighthouse, Result, LIGHTHOUSE_URL};
use tracing::info;
use tokio::time;
use std::{env, time::Duration};

async fn run(url: &str, auth: Authentication) -> Result<()> {
    let mut lh = Lighthouse::connect_with_tokio_to(url, auth).await?;
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

    let url = env::var("LIGHTHOUSE_URL").unwrap_or_else(|_| LIGHTHOUSE_URL.to_owned());
    let username = env::var("LIGHTHOUSE_USER").unwrap();
    let token = env::var("LIGHTHOUSE_TOKEN").unwrap();
    let auth = Authentication::new(username.as_str(), token.as_str());

    run(&url, auth).await.unwrap();
}
