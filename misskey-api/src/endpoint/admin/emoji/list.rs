use crate::model::{emoji::Emoji, id::Id};

use serde::Serialize;
use typed_builder::TypedBuilder;

#[derive(Serialize, Default, Debug, Clone, TypedBuilder)]
#[serde(rename_all = "camelCase")]
#[builder(doc)]
pub struct Request {
    #[cfg(feature = "12-48-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-48-0")))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option, into))]
    pub query: Option<String>,
    /// 1 .. 100
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub limit: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub since_id: Option<Id<Emoji>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub until_id: Option<Id<Emoji>>,
}

impl misskey_core::Request for Request {
    type Response = Vec<Emoji>;
    const ENDPOINT: &'static str = "admin/emoji/list";
}

impl_pagination!(Request, Emoji);

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let client = TestClient::new();
        client.admin.test(Request::default()).await;
    }

    #[tokio::test]
    async fn request_with_options() {
        let client = TestClient::new();
        client
            .admin
            .test(Request {
                #[cfg(feature = "12-48-0")]
                query: Some("a".to_string()),
                limit: Some(100),
                since_id: None,
                until_id: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_paginate() {
        let client = TestClient::new();
        let image_url = client.avatar_url().await;
        let emoji_id = client.admin.add_emoji_from_url(image_url).await;

        client
            .admin
            .test(Request {
                #[cfg(feature = "12-48-0")]
                query: None,
                limit: None,
                since_id: Some(emoji_id.clone()),
                until_id: Some(emoji_id.clone()),
            })
            .await;
    }
}
