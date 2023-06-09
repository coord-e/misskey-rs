use crate::model::channel::Channel;
#[cfg(feature = "12-48-0")]
use crate::model::id::Id;

use serde::Serialize;
use typed_builder::TypedBuilder;

#[derive(Serialize, Default, Debug, Clone, TypedBuilder)]
#[serde(rename_all = "camelCase")]
#[builder(doc)]
pub struct Request {
    /// 1 .. 100
    #[cfg(feature = "12-48-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-48-0")))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub limit: Option<u8>,
    #[cfg(feature = "12-48-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-48-0")))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub until_id: Option<Id<Channel>>,
    #[cfg(feature = "12-48-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-48-0")))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub since_id: Option<Id<Channel>>,
}

impl misskey_core::Request for Request {
    type Response = Vec<Channel>;
    const ENDPOINT: &'static str = "channels/followed";
}

#[cfg(feature = "12-48-0")]
impl_pagination!(Request, Channel);

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let client = TestClient::new();
        client.test(Request::default()).await;
    }

    #[cfg(feature = "12-48-0")]
    #[tokio::test]
    async fn request_with_limit() {
        let client = TestClient::new();
        client
            .test(Request {
                limit: Some(10),
                until_id: None,
                since_id: None,
            })
            .await;
    }

    #[cfg(feature = "12-48-0")]
    #[tokio::test]
    async fn request_paginate() {
        let client = TestClient::new();
        let channel = client
            .test(
                crate::endpoint::channels::create::Request::builder()
                    .name("test channel")
                    .build(),
            )
            .await;
        client
            .test(crate::endpoint::channels::follow::Request {
                channel_id: channel.id.clone(),
            })
            .await;

        client
            .test(Request {
                limit: None,
                until_id: Some(channel.id.clone()),
                since_id: Some(channel.id.clone()),
            })
            .await;
    }
}
