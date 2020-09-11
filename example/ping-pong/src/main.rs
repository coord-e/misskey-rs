use anyhow::{Context, Result};
use futures::stream::StreamExt;
use misskey::model::note::Note;
use misskey::streaming::channel::main::{self, MainStreamEvent};
use misskey::{Client, WebSocketClientBuilder};
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
    let client = WebSocketClientBuilder::new(opt.url)
        .auto_reconnect()
        .token(opt.i)
        .connect()
        .await?;

    // Connect to the main stream.
    // Main is a channel that streams events about the connected account, such as notifications.
    let mut stream = client
        .channel(main::Request::default())
        .await
        .context("Failed to connect to the main stream")?;

    loop {
        // Wait for the next note using `next` method from `StreamExt`.
        // We casually call `unwrap` on the returned `Option` here because our stream is infinite.
        let event = stream
            .next()
            .await
            .unwrap()
            .context("Failed to obtain the next event")?;

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
                client
                    .request(
                        // Build a `Request` to `notes/create` using a builder.
                        // Here, we specify the note's text and replying note id.
                        misskey::endpoint::notes::create::Request::builder()
                            .text("pong")
                            .reply_id(note_id)
                            .build(),
                    )
                    .await
                    .context("Failed to call an API")?
                    .into_result()
                    .context("Misskey API returned an error")?;
            }
            // other events are just ignored
            _ => {}
        }
    }
}
