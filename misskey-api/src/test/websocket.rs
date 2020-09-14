use crate::test::env;

use misskey_websocket::{WebSocketClient, WebSocketClientBuilder};

pub struct TestClient {
    pub admin: WebSocketClient,
    pub user: WebSocketClient,
}

impl TestClient {
    pub async fn new() -> Self {
        env::init_logger();

        let admin = WebSocketClientBuilder::new(env::TEST_WEBSOCKET_URL.clone())
            .token(env::TEST_ADMIN_TOKEN.clone())
            .auto_reconnect()
            .connect()
            .await
            .unwrap();
        let user = WebSocketClientBuilder::new(env::TEST_WEBSOCKET_URL.clone())
            .token(env::TEST_USER_TOKEN.clone())
            .auto_reconnect()
            .connect()
            .await
            .unwrap();

        TestClient { admin, user }
    }
}

impl std::ops::Deref for TestClient {
    type Target = WebSocketClient;
    fn deref(&self) -> &WebSocketClient {
        &self.user
    }
}
