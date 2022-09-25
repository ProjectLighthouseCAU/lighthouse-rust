use async_std::task;
use lighthouse_client::{Connection, Authentication, LighthouseResult};
use tracing::info;
use tracing_subscriber::FmtSubscriber;
use std::env;

async fn run(auth: Authentication) -> LighthouseResult<()> {
    let mut conn = Connection::new(auth).await?;
    info!("Connected to the Lighthouse server");

    // Required to get input events
    conn.request_stream().await?;

    loop {
        let event = conn.receive_input_event().await?;
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
