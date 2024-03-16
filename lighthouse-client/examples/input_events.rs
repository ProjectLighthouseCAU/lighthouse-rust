use clap::Parser;
use futures::StreamExt;
use lighthouse_client::{Lighthouse, Result, LIGHTHOUSE_URL, protocol::{Authentication, Payload}};
use tracing::info;

async fn run(url: &str, auth: Authentication) -> Result<()> {
    let mut lh = Lighthouse::connect_with_tokio_to(url, auth).await?;
    info!("Connected to the Lighthouse server");

    // Stream input events
    let mut stream = lh.stream_model().await?;
    while let Some(msg) = stream.next().await {
        if let Payload::InputEvent(event) = msg.payload {
            info!("Got input event: {:?}", event)
        }
    }

    Ok(())
}

#[derive(Parser)]
struct Args {
    /// The username.
    #[arg(short, long, env = "LIGHTHOUSE_USER")]
    username: String,
    /// The API token.
    #[arg(short, long, env = "LIGHTHOUSE_TOKEN")]
    token: String,
    /// The server URL.
    #[arg(long, env = "LIGHTHOUSE_URL", default_value = LIGHTHOUSE_URL)]
    url: String,
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    tracing_subscriber::fmt().init();
    _ = dotenvy::dotenv();

    let args = Args::parse();
    let auth = Authentication::new(&args.username, &args.token);

    run(&args.url, auth).await.unwrap();
}
