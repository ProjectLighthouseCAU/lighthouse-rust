use async_std::task;
use lighthouse_client::{Lighthouse, Authentication, LighthouseResult, Frame};
use tracing::info;
use tracing_subscriber::FmtSubscriber;
use std::{env, time::Duration};

async fn run(auth: Authentication) -> LighthouseResult<()> {
    let mut lh = Lighthouse::connect(auth).await?;
    info!("Connected to the Lighthouse server");

    loop {
        lh.put_frame(Frame::fill(rand::random())).await?;
        info!("Sent frame");

        task::sleep(Duration::from_secs(1)).await;
    }
}

fn main() {
    tracing::subscriber::set_global_default(FmtSubscriber::new()).unwrap();

    let username = env::var("LIGHTHOUSE_USER").unwrap();
    let token = env::var("LIGHTHOUSE_TOKEN").unwrap();
    let auth = Authentication::new(username.as_str(), token.as_str());

    task::block_on(run(auth)).unwrap();
}
