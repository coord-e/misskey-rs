use crate::model::following::FollowingWithFollowee;
use crate::model::user::UserId;

use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum Request {
    #[serde(rename_all = "camelCase")]
    WithUserId {
        user_id: UserId,
        /// 1 .. 100
        #[serde(skip_serializing_if = "Option::is_none")]
        limit: Option<u8>,
        #[serde(skip_serializing_if = "Option::is_none")]
        since_id: Option<UserId>,
        #[serde(skip_serializing_if = "Option::is_none")]
        until_id: Option<UserId>,
    },
    #[serde(rename_all = "camelCase")]
    WithUsername {
        username: String,
        host: Option<String>,
        /// 1 .. 100
        #[serde(skip_serializing_if = "Option::is_none")]
        limit: Option<u8>,
        #[serde(skip_serializing_if = "Option::is_none")]
        since_id: Option<UserId>,
        #[serde(skip_serializing_if = "Option::is_none")]
        until_id: Option<UserId>,
    },
}

impl misskey_core::Request for Request {
    type Response = Vec<FollowingWithFollowee>;
    const ENDPOINT: &'static str = "users/following";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request_with_id() {
        let mut client = TestClient::new();
        let user = client.me().await;

        client
            .test(Request::WithUserId {
                user_id: user.id,
                limit: None,
                since_id: None,
                until_id: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_with_username() {
        let mut client = TestClient::new();
        let user = client.me().await;

        client
            .test(Request::WithUsername {
                username: user.username,
                host: None,
                limit: None,
                since_id: None,
                until_id: None,
            })
            .await;
    }

    // TODO: request_with_username_and_host

    #[tokio::test]
    async fn request_with_limit() {
        let mut client = TestClient::new();
        let user = client.me().await;
        client
            .test(Request::WithUserId {
                user_id: user.id,
                limit: Some(100),
                since_id: None,
                until_id: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_paginate() {
        let mut client = TestClient::new();
        let user = client.user.me().await;
        let (new_user, _) = client.admin.create_user().await;
        client
            .user
            .test(crate::endpoint::following::create::Request {
                user_id: new_user.id.clone(),
            })
            .await;

        client
            .user
            .test(Request::WithUserId {
                user_id: user.id,
                limit: None,
                since_id: Some(new_user.id.clone()),
                until_id: Some(new_user.id.clone()),
            })
            .await;
    }
}
