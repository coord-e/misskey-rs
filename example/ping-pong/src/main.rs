use anyhow::Result;
use futures::stream::TryStreamExt;
use misskey::model::note::Note;
use misskey::prelude::*;
use misskey::streaming::channel::main::MainStreamEvent;
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

    // Build a client and connect to Misskey.
    let client = WebSocketClient::builder(opt.url)
        .token(opt.i)
        .connect()
        .await?;

    // Connect to the main stream.
    // the main stream is a channel that streams events about the connected account, such as notifications.
    let mut stream = client.main_stream().await?;

    // Wait for the next event using `try_next` method from `TryStreamExt`.
    while let Some(event) = stream.try_next().await? {
        match event {
            // Handle `Mention` event and extract inner `Note`
            MainStreamEvent::Mention(Note {
                id: note_id,
                text: Some(text),
                user,
                ..
            }) if text.contains("ping") => {
                println!("got ping from @{}", user.username);

                // Create a pong note as a reply to the mention
                client.reply(note_id, "pong").await?;
            }
            // other events are just ignored
            _ => {}
        }
    }

    Ok(())
}
