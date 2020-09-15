# misskey-rs

[![crates.io](https://img.shields.io/crates/v/misskey?style=flat-square)](https://crates.io/crates/misskey)
[![docs.rs](https://img.shields.io/badge/docs.rs-misskey-blue?style=flat-square)](https://docs.rs/misskey)
[![Actions Status](https://img.shields.io/github/workflow/status/coord-e/misskey-rs/CI?style=flat-square)](https://github.com/coord-e/misskey-rs/actions?workflow=CI)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg?style=flat-square)](http://makeapullrequest.com)

`misskey-rs` is an asynchronous [Misskey](https://github.com/syuilo/misskey) client library for Rust.

```rust
use misskey::{Client, HttpClient};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  let client = HttpClient::builder("https://your.instance.example/api/".parse()?)
      .token("API_TOKEN".to_string())
      .build()?;

  client
      .request(
          misskey::endpoint::notes::create::Request::builder()
              .text("Hello, Misskey")
              .build(),
      )
      .await?
      .into_result()?;

  Ok(())
}
```

Take a look at the [example](https://github.com/coord-e/misskey-rs/tree/develop/example) directory for more examples and detailed explanations.

## Usage

Add the following to your `Cargo.toml`:

```toml
[dependencies]
misskey = "0.1"
```

See the [API documentation](https://docs.rs/misskey) for further details.

## License

Licensed under either of

 * Apache License, Version 2.0
    ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
		([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
