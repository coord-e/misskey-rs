use std::fmt::Display;
use std::future::Future;
use std::sync::Once;
use std::time::Duration;

use anyhow::{anyhow, Result};
#[cfg(feature = "misskey-http")]
use misskey_http::HttpClient;
#[cfg(feature = "misskey-websocket")]
use misskey_websocket::WebSocketClient;

pub mod env {
    use url::Url;

    fn env_url(name: &str) -> Url {
        let url = std::env::var(name).unwrap();
        Url::parse(&url).unwrap()
    }

    fn env_token(name: &str) -> String {
        std::env::var(name).unwrap()
    }

    pub fn api_url() -> Url {
        env_url("TEST_API_URL")
    }

    pub fn websocket_url() -> Url {
        env_url("TEST_WEBSOCKET_URL")
    }

    pub fn admin_token() -> String {
        env_token("TEST_ADMIN_TOKEN")
    }

    pub fn user_token() -> String {
        env_token("TEST_USER_TOKEN")
    }

    pub fn token() -> String {
        env_token("TEST_USER_TOKEN")
    }
}

static INIT_LOGGER: Once = Once::new();

pub fn init_logger() {
    INIT_LOGGER.call_once(env_logger::init);
}

#[cfg(feature = "misskey-http")]
pub fn test_http_client(token: String) -> Result<HttpClient> {
    init_logger();
    let client = HttpClient::new(env::api_url(), Some(token))?;
    Ok(client)
}

#[cfg(feature = "misskey-websocket")]
pub async fn test_websocket_client(token: String) -> Result<WebSocketClient> {
    init_logger();

    let client = WebSocketClient::builder(env::websocket_url())
        .token(token)
        .auto_reconnect()
        .connect()
        .await?;

    Ok(client)
}

#[cfg(feature = "misskey-http")]
pub async fn test_client() -> Result<HttpClient> {
    test_http_client(env::token())
}

#[cfg(feature = "misskey-http")]
pub async fn test_admin_client() -> Result<HttpClient> {
    test_http_client(env::admin_token())
}

/// ```
/// use std::time::Duration;
/// use tokio::time::delay_for;
/// use misskey_test::persist;
/// use anyhow::{anyhow, Error};
/// #[tokio::main]
/// async fn main() {
///     assert!(persist(Duration::from_millis(10), async { delay_for(Duration::from_millis(5)).await; Ok::<(), Error>(()) }).await.is_err());
///     assert!(persist(Duration::from_millis(10), async { delay_for(Duration::from_millis(5)).await; Err(anyhow!("whoa")) }).await.is_err());
///     assert!(persist(Duration::from_millis(10), async { delay_for(Duration::from_millis(15)).await; Ok::<(), Error>(()) }).await.is_ok());
///     assert!(persist(Duration::from_millis(10), async { delay_for(Duration::from_millis(15)).await; Err(anyhow!("whoa")) }).await.is_ok());
/// }
/// ```
pub async fn persist<T, E>(duration: Duration, future: T) -> Result<()>
where
    T: Future<Output = std::result::Result<(), E>>,
    E: Display,
{
    match tokio::time::timeout(duration, future).await {
        Err(_) => Ok(()),
        Ok(Ok(())) => Err(anyhow!("unexpected success")),
        Ok(Err(e)) => Err(anyhow!("unexpected error: {}", e)),
    }
}
