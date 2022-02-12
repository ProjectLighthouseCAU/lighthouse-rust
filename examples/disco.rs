use async_std::task;
use lighthouse_client::{Color, Connection, Authentication, LighthouseResult, Display};
use rand::prelude::*;
use std::{env, time::Duration};

fn random_color() -> Color {
    let mut rng = thread_rng();
    Color::new(rng.gen(), rng.gen(), rng.gen())
}

async fn run(auth: Authentication) -> LighthouseResult<()> {
    let mut conn = Connection::new(auth).await?;
    
    loop {
        conn.send_display(Display::fill(random_color())).await?;
        task::sleep(Duration::from_secs(1)).await;
    }
}

fn main() {
    let username = env::var("LIGHTHOUSE_USERNAME").unwrap();
    let token = env::var("LIGHTHOUSE_TOKEN").unwrap();
    let auth = Authentication::new(username.as_str(), token.as_str());
    task::block_on(run(auth)).unwrap();
}
