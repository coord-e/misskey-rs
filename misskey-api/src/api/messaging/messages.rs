use crate::model::{
    messaging::{MessagingMessage, MessagingMessageId},
    user::UserId,
    user_group::UserGroupId,
};

use misskey_core::ApiRequest;
use serde::Serialize;

pub mod create;
pub mod delete;
pub mod read;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mark_as_read: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<UserId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<UserGroupId>,
    /// 1 .. 100
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since_id: Option<MessagingMessageId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until_id: Option<MessagingMessageId>,
}

impl ApiRequest for Request {
    type Response = Vec<MessagingMessage>;
    const ENDPOINT: &'static str = "messaging/messages";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request_with_user() {
        let mut client = TestClient::new();
        let admin = client.admin.me().await;
        client
            .user
            .test(Request {
                mark_as_read: None,
                user_id: Some(admin.id),
                group_id: None,
                limit: None,
                since_id: None,
                until_id: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_with_group() {
        let mut client = TestClient::new();
        let group = client
            .test(crate::api::users::groups::create::Request {
                name: "test".to_string(),
            })
            .await;
        client
            .test(Request {
                mark_as_read: None,
                user_id: None,
                group_id: Some(group.id),
                limit: None,
                since_id: None,
                until_id: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_with_option() {
        let mut client = TestClient::new();
        let admin = client.admin.me().await;
        client
            .user
            .test(Request {
                mark_as_read: Some(false),
                user_id: Some(admin.id),
                group_id: None,
                limit: None,
                since_id: None,
                until_id: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_with_limit() {
        let mut client = TestClient::new();
        let admin = client.admin.me().await;
        client
            .test(Request {
                mark_as_read: None,
                user_id: Some(admin.id),
                group_id: None,
                limit: Some(100),
                since_id: None,
                until_id: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_paginate() {
        let mut client = TestClient::new();
        let admin = client.admin.me().await;
        let user = client.user.me().await;
        let message = client
            .admin
            .test(crate::api::messaging::messages::create::Request {
                text: Some("hi".to_string()),
                user_id: Some(user.id),
                group_id: None,
                file_id: None,
            })
            .await;

        client
            .user
            .test(Request {
                mark_as_read: None,
                user_id: Some(admin.id),
                group_id: None,
                limit: None,
                since_id: Some(message.id.clone()),
                until_id: Some(message.id.clone()),
            })
            .await;
    }
}
