#[cfg(feature = "12-70-0")]
use crate::model::channel::Channel;
use crate::model::{id::Id, note::Note, user::User};

use serde::Serialize;
use typed_builder::TypedBuilder;

#[derive(Serialize, Debug, Clone, TypedBuilder)]
#[serde(rename_all = "camelCase")]
#[builder(doc)]
pub struct Request {
    #[builder(setter(into))]
    pub query: String,
    #[builder(default, setter(strip_option))]
    pub user_id: Option<Id<User>>,
    #[cfg(feature = "12-70-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-70-0")))]
    #[builder(default, setter(strip_option))]
    pub channel_id: Option<Id<Channel>>,
    #[cfg_attr(feature = "13-12-0", serde(skip_serializing_if = "Option::is_none"))]
    #[builder(default, setter(strip_option, into))]
    pub host: Option<String>,
    /// 1 .. 100
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub limit: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub since_id: Option<Id<Note>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub until_id: Option<Id<Note>>,
}

impl misskey_core::Request for Request {
    type Response = Vec<Note>;
    const ENDPOINT: &'static str = "notes/search";
}

impl_pagination!(Request, Note);

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request_with_query() {
        let client = TestClient::new();
        client
            .test(Request {
                query: "query".to_string(),
                user_id: None,
                #[cfg(feature = "12-70-0")]
                channel_id: None,
                host: None,
                limit: None,
                since_id: None,
                until_id: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_with_user_id() {
        let client = TestClient::new();
        let user = client.admin.me().await;
        client
            .test(Request {
                query: "query".to_string(),
                user_id: Some(user.id),
                #[cfg(feature = "12-70-0")]
                channel_id: None,
                host: None,
                limit: None,
                since_id: None,
                until_id: None,
            })
            .await;
    }

    #[cfg(feature = "12-70-0")]
    #[tokio::test]
    async fn request_with_channel_id() {
        let client = TestClient::new();
        let channel = client
            .test(
                crate::endpoint::channels::create::Request::builder()
                    .name("test channel")
                    .build(),
            )
            .await;

        client
            .test(Request {
                query: "query".to_string(),
                user_id: None,
                channel_id: Some(channel.id),
                host: None,
                limit: None,
                since_id: None,
                until_id: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_with_host() {
        let client = TestClient::new();
        client
            .test(Request {
                query: "query".to_string(),
                user_id: None,
                #[cfg(feature = "12-70-0")]
                channel_id: None,
                // TODO: proper host name
                host: Some("dummy".to_string()),
                limit: None,
                since_id: None,
                until_id: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_with_limit() {
        let client = TestClient::new();
        client
            .test(Request {
                query: "query".to_string(),
                user_id: None,
                #[cfg(feature = "12-70-0")]
                channel_id: None,
                host: None,
                limit: Some(100),
                since_id: None,
                until_id: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_paginate() {
        let client = TestClient::new();
        let note = client.create_note(Some("test"), None, None).await;

        client
            .test(Request {
                query: "test".to_string(),
                user_id: None,
                #[cfg(feature = "12-70-0")]
                channel_id: None,
                host: None,
                limit: None,
                since_id: Some(note.id.clone()),
                until_id: Some(note.id.clone()),
            })
            .await;
    }
}
