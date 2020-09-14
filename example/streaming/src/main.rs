use anyhow::{Context, Result};
use futures::stream::StreamExt;
use misskey::{Client, WebSocketClient};
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
        client
            .request(
                // Each endpoint implementation has a corresponding `Request` type.
                // We can dispatch an API call by passing `Request` to `Client::request` method.
                misskey::endpoint::notes::create::Request::builder()
                    .text(text)
                    .build(),
            )
            .await
            // `Client::request` method returns `Result<ApiResult<T>>`.
            // The returned `Result` may contain an error happened on our side
            // (e.g. networking failure or deserialization error)
            .context("Failed to call an API")?
            // Convert `ApiResult<T>` to `Result<T, ApiError>` using `ApiResult::into_result`.
            // `ApiError` is an error which is returned from Misskey API.
            .into_result()
            .context("Misskey API returned an error")?;
    }
}

// Print notes on the local timeline
async fn timeline(client: &WebSocketClient) -> Result<()> {
    use misskey::streaming::channel::local_timeline::{self, LocalTimelineEvent};

    // Connect to the local timeline.
    // `WebSocketClient::channel` returns a stream which is an instance of `Stream` and `Sink`.
    // We can communicate with the connected channel via those instances.
    let mut stream = client
        .channel(local_timeline::Request::default())
        .await
        .context("Failed to connect to the local timeline")?;

    loop {
        // Wait for the next note using `next` method from `StreamExt`.
        // We casually call `unwrap` on the returned `Option` here because our stream is infinite.
        let LocalTimelineEvent::Note(note) = stream
            .next()
            .await
            .unwrap()
            .context("Failed to obtain the next note")?;

        // `note` here has a type `misskey::model::note::Note`.
        println!(
            "<@{}> {}",
            note.user.username,
            note.text.unwrap_or_default()
        );
    }
}
