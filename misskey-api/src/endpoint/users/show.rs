use crate::model::user::{User, UserId};

use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum Request {
    #[serde(rename_all = "camelCase")]
    WithUserId { user_id: UserId },
    #[serde(rename_all = "camelCase")]
    WithUsername {
        username: String,
        host: Option<String>,
    },
}

impl misskey_core::Request for Request {
    type Response = User;
    const ENDPOINT: &'static str = "users/show";
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RequestWithUserIds {
    pub user_ids: Vec<UserId>,
}

impl misskey_core::Request for RequestWithUserIds {
    type Response = Vec<User>;
    const ENDPOINT: &'static str = "users/show";
}

#[cfg(test)]
mod tests {
    use super::{Request, RequestWithUserIds};
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request_with_user_id() {
        let mut client = TestClient::new();
        let user = client.me().await;

        client.test(Request::WithUserId { user_id: user.id }).await;
    }

    #[tokio::test]
    async fn request_with_username() {
        let mut client = TestClient::new();
        let user = client.me().await;

        client
            .test(Request::WithUsername {
                username: user.username,
                host: None,
            })
            .await;
    }

    // TODO: request_with_username_and_host

    #[tokio::test]
    async fn request_with_user_ids() {
        let mut client = TestClient::new();
        let user = client.user.me().await;
        let admin = client.admin.me().await;

        client
            .test(RequestWithUserIds {
                user_ids: vec![user.id, admin.id],
            })
            .await;
    }
}
