use derive_more::{Display, Error, From};
use futures::never::Never;
use futures::stream::StreamExt;
use misskey::Client;
use misskey::{WebSocketClient, WebSocketClientBuilder};
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
    Api(#[error(not(source))] misskey::model::ApiError),
    #[display(fmt = "WebSocket error: {}", _0)]
    Client(#[error(source)] misskey::websocket::Error),
}

async fn post(client: &WebSocketClient) -> Result<Never, Error> {
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
            .request(
                misskey::endpoint::notes::create::Request::builder()
                    .text(text)
                    .build(),
            )
            .await?
            .into_result()?;
    }
}

async fn timeline(client: &WebSocketClient) -> Result<Never, Error> {
    use misskey::streaming::channel;

    // subscribe to the timeline
    let mut stream = client
        .channel(channel::local_timeline::Request::default())
        .await?;

    loop {
        // wait for the next note
        let channel::local_timeline::LocalTimelineEvent::Note(note) =
            stream.next().await.unwrap()?;
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
        .auto_reconnect()
        .token(opt.i)
        .connect()
        .await?;

    // run two tasks simultaneously
    futures::try_join!(post(&client), timeline(&client))?;

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
