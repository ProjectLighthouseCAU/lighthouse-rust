use async_std::task;
use lighthouse_client::{Connection, Authentication, LighthouseResult, Display};
use log::{info, Level};
use std::{env, time::Duration};

async fn run(auth: Authentication) -> LighthouseResult<()> {
    let mut conn = Connection::new(auth).await?;
    info!("Connected to the Lighthouse server");

    loop {
        conn.send_display(Display::fill(rand::random())).await?;
        info!("Sent display");

        task::sleep(Duration::from_secs(1)).await;
    }
}

fn main() {
    simple_logger::init_with_level(Level::Info).unwrap();

    let username = env::var("LIGHTHOUSE_USER").unwrap();
    let token = env::var("LIGHTHOUSE_TOKEN").unwrap();
    let auth = Authentication::new(username.as_str(), token.as_str());

    task::block_on(run(auth)).unwrap();
}
