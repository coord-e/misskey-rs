use url::Url;

fn env_url(name: &str) -> Url {
    let url = std::env::var(name).unwrap();
    Url::parse(&url).unwrap()
}

fn env_token(name: &str) -> String {
    std::env::var(name).unwrap()
}

lazy_static::lazy_static! {
    pub static ref TEST_API_URL: Url = env_url("TEST_API_URL");
    pub static ref TEST_WEBSOCKET_URL: Url = env_url("TEST_WEBSOCKET_URL");
    pub static ref TEST_ADMIN_TOKEN: String = env_token("TEST_ADMIN_TOKEN");
    pub static ref TEST_USER_TOKEN: String = env_token("TEST_USER_TOKEN");
}
