use clap::Parser;
use lighthouse_client::{protocol::Authentication, Lighthouse, Result, TokioWebSocket, LIGHTHOUSE_URL};
use tracing::info;

async fn run(mut lh: Lighthouse<TokioWebSocket>) -> Result<()> {
    info!("Connected to the Lighthouse server");

    info!("Creating test directory...");
    _ = lh.mkdir(&["test"]).await;
    info!("Tree: {}", lh.list(&["test"]).await?.payload);

    info!("Posting to test directory...");
    lh.post(&["test", "a", "b", "c"], "Hello world!".to_string()).await?.payload;
    info!("Tree: {}", lh.list(&["test"]).await?.payload);

    info!("Getting test resource...");
    let result: String = lh.get(&["test", "a", "b", "c"]).await?.payload;
    info!("Result: {:?}", result);
    info!("Tree: {}", lh.list(&["test"]).await?.payload);

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
async fn main() -> Result<()> {
    tracing_subscriber::fmt().init();
    _ = dotenvy::dotenv();

    let args = Args::parse();
    let auth = Authentication::new(&args.username, &args.token);
    let lh = Lighthouse::connect_with_tokio_to(&args.url, auth).await?;

    run(lh).await
}
