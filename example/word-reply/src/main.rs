use anyhow::Result;
use futures::stream::TryStreamExt;
use misskey::model::query::Query;
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
        .token(opt.i)
        .connect()
        .await?;

    // Create a new antenna.
    let antenna = client
        .build_antenna()
        .name("word-reply example")
        .include(Query::from_vec(
            opt.words.into_iter().map(|x| vec![x]).collect(),
        ))
        .case_sensitive(opt.case_sensitive)
        .create()
        .await?;

    // Connect to the antenna's timeline.
    let mut stream = client.antenna_timeline(&antenna).await?;

    // Wait for the next note using `try_next` method from `TryStreamExt`.
    while let Some(note) = stream.try_next().await? {
        println!("received a note from @{}", note.user.username);

        // Create a note as a reply to the note
        client.reply(&note, &opt.reply).await?;
    }

    Ok(())
}
