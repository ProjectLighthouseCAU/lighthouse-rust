use async_std::task;
use lighthouse_client::{Lighthouse, Authentication, LighthouseResult};
use tracing::info;
use tracing_subscriber::FmtSubscriber;
use std::env;

async fn run(auth: Authentication) -> LighthouseResult<()> {
    let mut lh = Lighthouse::connect(auth).await?;
    info!("Connected to the Lighthouse server");

    // Required to get input events
    lh.request_stream().await?;

    loop {
        let event = lh.receive_input_event().await?;
        info!("Got event: {:?}", event);
    }
}

fn main() {
    tracing::subscriber::set_global_default(FmtSubscriber::new()).unwrap();

    let username = env::var("LIGHTHOUSE_USER").unwrap();
    let token = env::var("LIGHTHOUSE_TOKEN").unwrap();
    let auth = Authentication::new(username.as_str(), token.as_str());

    task::block_on(run(auth)).unwrap();
}
