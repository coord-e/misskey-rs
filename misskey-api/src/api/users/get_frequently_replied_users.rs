use crate::model::user::{User, UserId};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub user_id: UserId,
    /// 1 .. 100
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u8>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Reply {
    pub user: User,
    pub weight: f64,
}

impl misskey_core::Request for Request {
    type Response = Vec<Reply>;
    const ENDPOINT: &'static str = "users/get-frequently-replied-users";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let mut client = TestClient::new();
        let user = client.user.me().await;
        let note = client.admin.create_note(Some("test"), None, None).await;
        client
            .user
            .create_note(Some("test"), None, Some(note.id))
            .await;

        client
            .user
            .test(Request {
                user_id: user.id,
                limit: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_with_limit() {
        let mut client = TestClient::new();
        let user = client.me().await;

        client
            .test(Request {
                user_id: user.id,
                limit: Some(100),
            })
            .await;
    }
}
