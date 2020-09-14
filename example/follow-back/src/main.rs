use anyhow::{Context, Result};
use futures::stream::StreamExt;
use misskey::streaming::channel::main::{self, MainStreamEvent};
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

    // Build a client and connect to Misskey.
    let client = WebSocketClient::builder(opt.url)
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
            // Handle `Followed` event and extract inner `User`
            MainStreamEvent::Followed(user) if !user.is_bot => {
                println!("followed from @{}", user.username);

                // Follow back `user`
                client
                    .request(misskey::endpoint::following::create::Request { user_id: user.id })
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
