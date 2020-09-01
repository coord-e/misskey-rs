use crate::model::channel::Channel;
#[cfg(feature = "12-47-2")]
use crate::model::channel::ChannelId;

use serde::Serialize;
use typed_builder::TypedBuilder;

#[derive(Serialize, Default, Debug, Clone, TypedBuilder)]
#[serde(rename_all = "camelCase")]
#[builder(doc)]
pub struct Request {
    /// 1 .. 100
    #[cfg(feature = "12-47-2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-47-2")))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub limit: Option<u8>,
    #[cfg(feature = "12-47-2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-47-2")))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub until_id: Option<ChannelId>,
    #[cfg(feature = "12-47-2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-47-2")))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub since_id: Option<ChannelId>,
}

impl misskey_core::Request for Request {
    type Response = Vec<Channel>;
    const ENDPOINT: &'static str = "channels/followed";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let mut client = TestClient::new();
        client.test(Request::default()).await;
    }

    #[cfg(feature = "12-47-2")]
    #[tokio::test]
    async fn request_with_limit() {
        let mut client = TestClient::new();
        client
            .test(Request {
                limit: Some(10),
                until_id: None,
                since_id: None,
            })
            .await;
    }

    #[cfg(feature = "12-47-2")]
    #[tokio::test]
    async fn request_paginate() {
        let mut client = TestClient::new();
        let channel = client
            .test(crate::endpoint::channels::create::Request {
                name: "test channel".to_string(),
                description: None,
                banner_id: None,
            })
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
