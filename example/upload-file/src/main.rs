use std::path::PathBuf;

use anyhow::{Context, Result};
use misskey::{Client, HttpClient};
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
    let client = HttpClient::new(opt.url, Some(opt.i))?;

    // Miscellaneous code to compute some necessary values
    let mime = mime_guess::from_path(&opt.file).first_or_octet_stream();
    let file_name = opt.file.file_name().unwrap().to_str().unwrap();

    // Upload a file to drive
    let file = client
        .request_with_file(
            // We use `HttpClient::request_with_file` to upload a file.
            misskey::endpoint::drive::files::create::Request {
                is_sensitive: Some(opt.sensitive),
                name: opt.name.clone(),
                // filling uninterested parameters with defaults
                ..Default::default()
            },
            mime,
            file_name,
            &opt.file,
        )
        .await
        .context("Failed to call an API")?
        .into_result()
        .context("Misskey API returned an error")?;

    // Create a note with uploaded file
    client
        .request(
            misskey::endpoint::notes::create::Request::builder()
                .file_ids(vec![file.id])
                .build(),
        )
        .await
        .context("Failed to call an API")?
        .into_result()
        .context("Misskey API returned an error")?;

    Ok(())
}
