use anyhow::Result;
use futures::stream::TryStreamExt;
use misskey::prelude::*;
use misskey::WebSocketClient;
use structopt::StructOpt;
use url::Url;

#[derive(StructOpt)]
struct Opt {
    #[structopt(short, long, parse(try_from_str = Url::parse))]
    url: Url,
    #[structopt(env = "API_TOKEN")]
    i: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let opt = Opt::from_args();

    // Configure and build a client using `WebSocketClientBuilder`.
    let client = WebSocketClient::builder(opt.url)
        .auto_reconnect()
        .token(opt.i)
        .connect()
        .await?;

    // Run two asynchronous tasks simultaneously.
    futures::try_join!(post(&client), timeline(&client))?;

    Ok(())
}

// Post what you entered
async fn post(client: &WebSocketClient) -> Result<()> {
    // We use async I/O from `tokio` for now
    use tokio::io::{self, AsyncBufReadExt, BufReader};

    let mut stdin = BufReader::new(io::stdin());

    loop {
        let mut text = String::new();

        // Wait for user input
        stdin.read_line(&mut text).await?;

        if text.trim().is_empty() {
            continue;
        }

        // Create a note containing `text` as its text
        client.create_note(text).await?;
    }
}

// Print notes on the local timeline
async fn timeline(client: &WebSocketClient) -> Result<()> {
    // Connect to the local timeline.
    let mut stream = client.local_timeline().await?;

    // Wait for the next note using `try_next` method from `TryStreamExt`.
    while let Some(note) = stream.try_next().await? {
        // `note` here has a type `misskey::model::note::Note`.
        println!(
            "<@{}> {}",
            note.user.username,
            note.text.unwrap_or_default()
        );
    }

    Ok(())
}
