use std::path::PathBuf;

use anyhow::Result;
use misskey::prelude::*;
use misskey::HttpClient;
use structopt::StructOpt;
use url::Url;

#[derive(StructOpt)]
struct Opt {
    #[structopt(short, long, parse(try_from_str = Url::parse))]
    url: Url,
    #[structopt(short, long, parse(from_os_str))]
    file: PathBuf,
    #[structopt(short, long)]
    name: Option<String>,
    #[structopt(short, long)]
    sensitive: bool,
    #[structopt(env = "API_TOKEN")]
    i: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let opt = Opt::from_args();

    // Create `HttpClient`.
    // Note that `HttpClient` can upload files while `WebSocketClient` can't.
    let client = HttpClient::with_token(opt.url, opt.i)?;

    // Upload a file to drive
    let mut builder = client.build_file(&opt.file);
    if let Some(name) = opt.name {
        builder.name(name);
    }
    let file = builder.sensitive(opt.sensitive).upload().await?;

    // Create a note with uploaded file
    client.build_note().attach_file(&file).create().await?;

    Ok(())
}
