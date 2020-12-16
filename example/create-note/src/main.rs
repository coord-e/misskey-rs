use anyhow::Result;
use misskey::prelude::*;
use misskey::HttpClient;
use structopt::StructOpt;
use url::Url;

#[derive(StructOpt)]
struct Opt {
    #[structopt(short, long)]
    text: String,
    #[structopt(short, long, parse(try_from_str = Url::parse))]
    url: Url,
    #[structopt(env = "API_TOKEN")]
    i: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let opt = Opt::from_args();

    // Create `HttpClient` with token `opt.i`
    let client = HttpClient::new(opt.url, Some(opt.i))?;

    // Create a note containing `opt.text` as a text
    client.create_note(opt.text).await?;

    Ok(())
}
