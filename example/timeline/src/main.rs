use std::sync::Arc;

use derive_more::{Display, Error, From};
use futures::lock::Mutex;
use futures::never::Never;
use futures::stream::StreamExt;
use misskey_core::{streaming::SubscriptionClient, Client};
use misskey_websocket::{WebSocketClient, WebSocketClientBuilder};
use structopt::StructOpt;
use url::Url;

#[derive(StructOpt)]
struct Opt {
    #[structopt(short, long, parse(try_from_str = Url::parse))]
    url: Url,
    #[structopt(env = "API_TOKEN")]
    i: String,
}

#[derive(Debug, Display, From, Error)]
enum Error {
    #[display(fmt = "IO error: {}", _0)]
    Io(#[error(source)] tokio::io::Error),
    #[display(fmt = "API error: {} ({})", "_0.message", "_0.id")]
    Api(#[error(not(source))] misskey_core::model::ApiError),
    #[display(fmt = "JSON error: {}", _0)]
    Client(#[error(source)] misskey_websocket::error::Error),
}

async fn post(client: Arc<Mutex<WebSocketClient>>) -> Result<Never, Error> {
    use tokio::io::{self, AsyncBufReadExt, BufReader};
    let mut stdin = BufReader::new(io::stdin());

    loop {
        let mut text = String::new();

        // wait for user input
        stdin.read_line(&mut text).await?;

        if text.trim().is_empty() {
            continue;
        }

        // create a note containing `text` as its text
        client
            .lock()
            .await
            .request(
                misskey_api::endpoint::notes::create::Request::builder()
                    .text(text)
                    .build(),
            )
            .await?
            .into_result()?;
    }
}

async fn timeline(client: Arc<Mutex<WebSocketClient>>) -> Result<Never, Error> {
    use misskey_api::streaming::channel;

    // subscribe to the timeline
    let mut stream = client
        .lock()
        .await
        .subscribe(channel::ConnectRequest::<
            channel::local_timeline::LocalTimeline,
        >::new())
        .await?;

    loop {
        // wait for the next note
        let note = stream.next().await.unwrap()?.into_inner().note;
        println!(
            "<@{}> {}",
            note.user.username,
            note.text.unwrap_or_default()
        );
    }
}

async fn run(opt: Opt) -> Result<(), Error> {
    // create the client with API token
    let client = WebSocketClientBuilder::new(opt.url)
        .token(opt.i)
        .connect()
        .await?;

    // wrap the client to share it between tasks
    let client = Arc::new(Mutex::new(client));

    // run two tasks simultaneously
    futures::try_join!(post(Arc::clone(&client)), timeline(client))?;

    // we can reason that we won't reach here from `Never` type, but omitted it for brevity
    Ok(())
}

#[tokio::main]
async fn main() {
    let opt = Opt::from_args();
    if let Err(e) = run(opt).await {
        eprintln!("error: {}", e);
        std::process::exit(1);
    }
}
