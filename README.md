# misskey-rs

[![crates.io](https://img.shields.io/crates/v/misskey)](https://crates.io/crates/misskey)
[![docs.rs](https://docs.rs/misskey/badge.svg)](https://docs.rs/misskey)
[![CI](https://github.com/coord-e/misskey-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/coord-e/misskey-rs/actions/workflows/ci.yml)

`misskey-rs` is an asynchronous [Misskey](https://github.com/misskey-dev/misskey) client library for Rust.

```rust
use misskey::prelude::*;
use misskey::HttpClient;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  let client = HttpClient::builder("https://your.instance.example/api/")
      .token("API_TOKEN")
      .build()?;

  client.create_note("Hello, Misskey").await?;

  Ok(())
}
```

Take a look at the [example](https://github.com/coord-e/misskey-rs/tree/develop/example) directory for more examples.

## Usage

Add the following to your `Cargo.toml`:

```toml
[dependencies]
misskey = { version = "0.3.0-rc.1", features = ["12-75-0"] }
```

To run the example above, you will need to add the following dependencies:

```toml
tokio = { version = "1.0", features = ["full"] }
anyhow = "1.0"
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
