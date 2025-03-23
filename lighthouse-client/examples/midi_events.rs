use clap::Parser;
use futures::StreamExt;
use lighthouse_client::{protocol::Authentication, Lighthouse, Result, TokioWebSocket, LIGHTHOUSE_URL};
use lighthouse_protocol::InputEvent;
use midi_msg::MidiMsg;
use tracing::{info, warn};

async fn run(lh: Lighthouse<TokioWebSocket>) -> Result<()> {
    info!("Connected to the Lighthouse server");

    // Stream input events
    let mut stream = lh.stream_input().await?;
    while let Some(msg) = stream.next().await {
        let event = msg?.payload;
        if let InputEvent::Midi(midi) = event {
            match MidiMsg::from_midi(&midi.data) {
                Ok((msg, _)) => info!("Got MIDI message: {:?}", msg),
                Err(e) => warn!("Could not parse MIDI message: {:?}", e),
            };
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
async fn main() -> Result<()> {
    tracing_subscriber::fmt().init();
    _ = dotenvy::dotenv();

    let args = Args::parse();
    let auth = Authentication::new(&args.username, &args.token);
    let lh = Lighthouse::connect_with_tokio_to(&args.url, auth).await?;

    run(lh).await
}
