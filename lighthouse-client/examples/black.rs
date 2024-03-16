use clap::Parser;
use lighthouse_client::{Lighthouse, Result, LIGHTHOUSE_URL};
use lighthouse_protocol::{Authentication, Color, Frame};
use tracing::info;

async fn run(url: &str, auth: Authentication) -> Result<()> {
    let mut lh = Lighthouse::connect_with_tokio_to(url, auth).await?;
    info!("Connected to the Lighthouse server");

    lh.put_model(Frame::fill(Color::BLACK)).await?;

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
