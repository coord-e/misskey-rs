use misskey_test::env;
use misskey_websocket::{WebSocketClient, WebSocketClientBuilder};

pub struct TestClient {
    pub admin: WebSocketClient,
    pub user: WebSocketClient,
}

impl TestClient {
    pub async fn new() -> Self {
        misskey_test::init_logger();

        let admin = WebSocketClientBuilder::new(env::websocket_url())
            .token(env::admin_token())
            .connect()
            .await
            .unwrap();
        let user = WebSocketClientBuilder::new(env::websocket_url())
            .token(env::user_token())
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
