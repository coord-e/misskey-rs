use anyhow::{Context, Result};
use futures::stream::StreamExt;
use misskey::model::{antenna::AntennaSource, query::Query};
use misskey::streaming::channel::antenna::{self, AntennaStreamEvent};
use misskey::{Client, WebSocketClient};
use structopt::StructOpt;
use url::Url;

#[derive(StructOpt)]
struct Opt {
    #[structopt(short, long, parse(try_from_str = Url::parse))]
    url: Url,
    #[structopt(env = "API_TOKEN")]
    i: String,
    #[structopt(short, long)]
    case_sensitive: bool,
    #[structopt(short, long)]
    words: Vec<String>,
    #[structopt(short, long)]
    reply: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let opt = Opt::from_args();

    // Build a client and connect to Misskey.
    let client = WebSocketClient::builder(opt.url)
        .auto_reconnect()
        .token(opt.i)
        .connect()
        .await?;

    // Create a new antenna.
    let antenna = client
        .request(
            misskey::endpoint::antennas::create::Request::builder()
                .name("word-reply example")
                .keywords(Query(opt.words.into_iter().map(|x| vec![x]).collect()))
                .src(AntennaSource::All)
                .users(vec![])
                .case_sensitive(opt.case_sensitive)
                .with_replies(true)
                .with_file(false)
                .notify(false)
                .build(),
        )
        .await
        .context("Failed to call an API")?
        .into_result()
        .context("Misskey API returned an error")?;

    // Connect to the antenna's stream.
    let mut stream = client
        .channel(antenna::Request {
            antenna_id: antenna.id,
        })
        .await
        .context("Failed to connect to the antenna stream")?;

    loop {
        // Wait for the next note using `next` method from `StreamExt`.
        let AntennaStreamEvent::Note(note) = stream
            .next()
            .await
            .unwrap()
            .context("Failed to obtain the next note")?;

        println!("received a note from @{}", note.user.username);

        // Create a note as a reply to the note
        client
            .request(
                // Build a `Request` to `notes/create` using a builder.
                misskey::endpoint::notes::create::Request::builder()
                    .text(opt.reply.clone())
                    .reply_id(note.id)
                    .build(),
            )
            .await
            .context("Failed to call an API")?
            .into_result()
            .context("Misskey API returned an error")?;
    }
}
