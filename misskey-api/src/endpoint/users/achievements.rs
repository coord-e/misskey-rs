use crate::model::{
    id::Id,
    user::{Achievement, User},
};

use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub user_id: Id<User>,
}

impl misskey_core::Request for Request {
    type Response = Vec<Achievement>;
    const ENDPOINT: &'static str = "users/achievements";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let client = TestClient::new();
        let user_id = client.me().await.id;

        client.test(Request { user_id }).await;
    }
}
