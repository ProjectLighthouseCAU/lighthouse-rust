use clap::Parser;
use lighthouse_client::{protocol::Authentication, root, Lighthouse, Result, LIGHTHOUSE_URL};
use tracing::info;

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
    /// Whether to list only the first layer.
    #[arg(short, long)]
    nonrecursive: bool,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().init();
    _ = dotenvy::dotenv();

    let args = Args::parse();
    let auth = Authentication::new(&args.username, &args.token);
    let lh = Lighthouse::connect_with_tokio_to(&args.url, auth).await?;

    let tree = if args.nonrecursive {
        lh.list_dir(root![]).await
    } else {
        lh.list_tree(root![]).await
    }?.payload;

    info!("Got {}", tree);

    Ok(())
}
