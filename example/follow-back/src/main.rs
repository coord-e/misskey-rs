use anyhow::Result;
use futures::stream::TryStreamExt;
use misskey::prelude::*;
use misskey::streaming::channel::main::MainStreamEvent;
use misskey::{HttpClient, WebSocketClient};
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

    // Create `HttpClient`.
    let http_client = HttpClient::with_token(opt.url.clone(), opt.i.clone())?;

    // Build `WebSocketClient` and connect to Misskey.
    let ws_client = WebSocketClient::builder(opt.url)
        .token(opt.i)
        .connect()
        .await?;

    // Connect to the main stream.
    // the main stream is a channel that streams events about the connected account, such as notifications.
    let mut stream = ws_client.main_stream().await?;

    // Wait for the next event using `try_next` method from `TryStreamExt`.
    while let Some(event) = stream.try_next().await? {
        match event {
            // Handle `Followed` event and extract inner `User`
            MainStreamEvent::Followed(user) if !user.is_bot => {
                println!("followed from @{}", user.username);

                // Follow back `user` if you haven't already.
                if !http_client.is_following(&user).await? {
                    http_client.follow(&user).await?;
                }
            }
            // other events are just ignored
            _ => {}
        }
    }

    Ok(())
}
