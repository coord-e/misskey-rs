use serde::Serialize;
use typed_builder::TypedBuilder;

use crate::model::{
    channel::{Channel, ChannelSearchType},
    id::Id,
};

#[derive(Serialize, Default, Debug, Clone, TypedBuilder)]
#[serde(rename_all = "camelCase")]
#[builder(doc)]
pub struct Request {
    #[builder(default, setter(into))]
    pub query: String,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub type_: Option<ChannelSearchType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub limit: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub since_id: Option<Id<Channel>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub until_id: Option<Id<Channel>>,
}

impl misskey_core::Request for Request {
    type Response = Vec<Channel>;
    const ENDPOINT: &'static str = "channels/search";
}

impl_pagination!(Request, Channel);

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::{
        model::channel::ChannelSearchType,
        test::{ClientExt, TestClient},
    };

    #[tokio::test]
    async fn request() {
        let client = TestClient::new();
        client
            .test(
                crate::endpoint::channels::create::Request::builder()
                    .name("test")
                    .build(),
            )
            .await;

        client.test(Request::default()).await;
    }

    #[tokio::test]
    async fn request_with_type() {
        let client = TestClient::new();
        client
            .test(
                crate::endpoint::channels::create::Request::builder()
                    .name("test")
                    .build(),
            )
            .await;

        client
            .test(Request {
                query: "test".to_string(),
                type_: Some(ChannelSearchType::NameAndDescription),
                limit: None,
                since_id: None,
                until_id: None,
            })
            .await;
        client
            .test(Request {
                query: "test".to_string(),
                type_: Some(ChannelSearchType::NameOnly),
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
            .test(
                crate::endpoint::channels::create::Request::builder()
                    .name("test")
                    .build(),
            )
            .await;

        client
            .test(Request {
                query: "test".to_string(),
                type_: None,
                limit: Some(100),
                since_id: None,
                until_id: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_paginate() {
        let client = TestClient::new();
        let channel = client
            .test(
                crate::endpoint::channels::create::Request::builder()
                    .name("test")
                    .build(),
            )
            .await;

        client
            .test(Request {
                query: "test".to_string(),
                type_: None,
                limit: None,
                since_id: Some(channel.id),
                until_id: Some(channel.id),
            })
            .await;
    }
}
