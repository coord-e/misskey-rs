use std::sync::Once;

use anyhow::{Context, Result};
use misskey_http::HttpClient;
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

pub fn test_http_client(token: String) -> Result<HttpClient> {
    init_logger();

    HttpClient::new(env::api_url(), Some(token)).context("Failed to initialize HttpClient")
}

pub async fn test_websocket_client(token: String) -> Result<WebSocketClient> {
    init_logger();

    WebSocketClient::builder(env::websocket_url())
        .token(token)
        .auto_reconnect()
        .connect()
        .await
        .context("Failed to connect with WebSocketClient")
}

pub async fn test_client() -> Result<HttpClient> {
    test_http_client(env::token())
}

pub async fn test_admin_client() -> Result<HttpClient> {
    test_http_client(env::admin_token())
}
