use crate::model::{id::Id, messaging::MessagingMessage, user::User, user_group::UserGroup};

use serde::Serialize;
use typed_builder::TypedBuilder;

pub mod create;
pub mod delete;
pub mod read;

#[derive(Serialize, Debug, Clone, TypedBuilder)]
#[serde(rename_all = "camelCase")]
#[builder(doc)]
pub struct Request {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub mark_as_read: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub user_id: Option<Id<User>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub group_id: Option<Id<UserGroup>>,
    /// 1 .. 100
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub limit: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub since_id: Option<Id<MessagingMessage>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub until_id: Option<Id<MessagingMessage>>,
}

impl misskey_core::Request for Request {
    type Response = Vec<MessagingMessage>;
    const ENDPOINT: &'static str = "messaging/messages";
}

impl_pagination!(Request, MessagingMessage);

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request_with_user() {
        let client = TestClient::new();
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
        let client = TestClient::new();
        let group = client
            .test(crate::endpoint::users::groups::create::Request {
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
        let client = TestClient::new();
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
        let client = TestClient::new();
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
        let client = TestClient::new();
        let admin = client.admin.me().await;
        let user = client.user.me().await;
        let message = client
            .admin
            .test(crate::endpoint::messaging::messages::create::Request {
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
