use crate::model::user::{UserId, UserRelation};

use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub user_id: UserId,
}

impl misskey_core::Request for Request {
    type Response = UserRelation;
    const ENDPOINT: &'static str = "users/relation";
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RequestWithUserIds {
    #[serde(rename = "userId")]
    pub user_ids: Vec<UserId>,
}

impl misskey_core::Request for RequestWithUserIds {
    type Response = Vec<UserRelation>;
    const ENDPOINT: &'static str = "users/relation";
}

#[cfg(test)]
mod tests {
    use super::{Request, RequestWithUserIds};
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let mut client = TestClient::new();
        let admin = client.admin.me().await;

        client.user.test(Request { user_id: admin.id }).await;
    }

    #[tokio::test]
    async fn request_with_user_ids() {
        let mut client = TestClient::new();
        let admin = client.admin.me().await;
        let (new_user, _) = client.admin.create_user().await;

        client
            .user
            .test(RequestWithUserIds {
                user_ids: vec![admin.id, new_user.id],
            })
            .await;
    }
}
