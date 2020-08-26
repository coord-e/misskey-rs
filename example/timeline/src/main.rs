use std::sync::Arc;

use derive_more::{Display, Error, From};
use futures::lock::Mutex;
use futures::never::Never;
use futures::stream::StreamExt;
use misskey_api::model::timeline::Timeline;
use misskey_core::{streaming::SubscriptionClient, Client};
use misskey_websocket::{WebSocketClient, WebSocketClientBuilder};
use structopt::StructOpt;
use url::Url;

#[derive(StructOpt)]
struct Opt {
    #[structopt(short, long, parse(try_from_str = Url::parse))]
    url: Url,
    #[structopt(short, long, default_value = "local", possible_values = &["global", "home", "social", "local"])]
    timeline: Timeline,
    #[structopt(env = "API_TOKEN")]
    i: String,
}

#[derive(Debug, Display, From, Error)]
enum Error {
    #[display(fmt = "IO error: {}", _0)]
    Io(#[error(source)] async_std::io::Error),
    #[display(fmt = "API error: {} ({})", "_0.message", "_0.id")]
    Api(#[error(not(source))] misskey_core::model::ApiError),
    #[display(fmt = "JSON error: {}", _0)]
    Client(#[error(source)] misskey_websocket::error::Error),
}

async fn post(client: Arc<Mutex<WebSocketClient>>) -> Result<Never, Error> {
    let stdin = async_std::io::stdin();

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
                misskey_api::api::notes::create::Request::builder()
                    .text(text)
                    .build(),
            )
            .await?
            .into_result()?;
    }
}

async fn timeline(client: Arc<Mutex<WebSocketClient>>, timeline: Timeline) -> Result<Never, Error> {
    // subscribe to the timeline
    let mut stream = client
        .lock()
        .await
        .subscribe(misskey_api::streaming::timeline::Request { channel: timeline })
        .await?;

    loop {
        // wait for the next note
        let note = stream.next().await.unwrap()?.body;
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
    futures::try_join!(post(Arc::clone(&client)), timeline(client, opt.timeline))?;

    // we can reason that we won't reach here from `Never` type, but omitted it for brevity
    Ok(())
}

fn main() {
    let opt = Opt::from_args();
    if let Err(e) = async_std::task::block_on(run(opt)) {
        eprintln!("error: {}", e);
        std::process::exit(1);
    }
}
