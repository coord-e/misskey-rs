use anyhow::{Context, Result};
use misskey::{Client, HttpClient};
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
    client
        .request(
            // Each endpoint implementation has a corresponding `Request` type.
            // We can dispatch an API call by passing `Request` to `Client::request` method.
            // Here, we build a `Request` to `notes/create` using a `Request::builder()`.
            misskey::endpoint::notes::create::Request::builder()
                .text(opt.text)
                .build(),
        )
        // Asynchronously wait for the response
        .await
        // `Client::request` method returns `Result<ApiResult<T>>`.
        // The returned `Result` may contain an error happened on our side
        // (e.g. networking failure or deserialization error)
        .context("Failed to call an API")?
        // Convert `ApiResult<T>` to `Result<T, ApiError>` using `ApiResult::into_result`.
        // `ApiError` is an error which is returned from Misskey API.
        .into_result()
        .context("Misskey API returned an error")?;

    Ok(())
}
