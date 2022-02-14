use async_std::task;
use lighthouse_client::{Connection, Authentication, LighthouseResult};
use log::{info, Level};
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
    simple_logger::init_with_level(Level::Info).unwrap();

    let username = env::var("LIGHTHOUSE_USERNAME").unwrap();
    let token = env::var("LIGHTHOUSE_TOKEN").unwrap();
    let auth = Authentication::new(username.as_str(), token.as_str());

    task::block_on(run(auth)).unwrap();
}
