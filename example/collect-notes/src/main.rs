use std::path::PathBuf;
use std::time::Duration;

use anyhow::Result;
use futures::stream::TryStreamExt;
use misskey::prelude::*;
use misskey::HttpClient;
use structopt::StructOpt;
use url::Url;

#[derive(StructOpt)]
struct Opt {
    #[structopt(short, long, parse(from_os_str))]
    output: PathBuf,
    #[structopt(short, long, parse(try_from_str = Url::parse))]
    url: Url,
    #[structopt(env = "API_TOKEN")]
    i: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let opt = Opt::from_args();

    // Create `HttpClient` with token `opt.i`
    let client = HttpClient::with_token(opt.url, opt.i)?;

    // Use async I/O from `tokio` for now
    use tokio::fs::File;
    use tokio::io::{AsyncWriteExt, BufWriter};

    // Create a file and writer
    let file = File::create(opt.output).await?;
    let mut writer = BufWriter::new(file);

    // `notes` variable here is a stream to enumerate all local notes
    let mut notes = client.local_notes(..);
    // Having an interval of 10 seconds between requests; this may be too much of concern,
    // so adjust it for the server you are targeting
    notes.set_interval(Duration::from_secs(10));
    // Fetch 100 notes at once
    notes.set_page_size(100);

    // Retrieve all notes until there are no more.
    while let Some(note) = notes.try_next().await? {
        if let Some(text) = note.text {
            // Write the note content to the file
            writer.write_all(text.as_bytes()).await?;
            writer.write_u8(b'\n').await?;
        }
    }

    writer.flush().await?;

    Ok(())
}
