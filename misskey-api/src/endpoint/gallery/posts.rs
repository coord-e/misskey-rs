use crate::model::{gallery::GalleryPost, id::Id};

use serde::Serialize;
use typed_builder::TypedBuilder;

pub mod create;
pub mod like;
pub mod show;
pub mod unlike;

#[cfg(feature = "12-79-2")]
#[cfg_attr(docsrs, doc(cfg(feature = "12-79-2")))]
pub mod delete;

#[cfg(feature = "12-79-2")]
#[cfg_attr(docsrs, doc(cfg(feature = "12-79-2")))]
pub mod update;

#[derive(Serialize, Default, Debug, Clone, TypedBuilder)]
#[serde(rename_all = "camelCase")]
#[builder(doc)]
pub struct Request {
    /// 1 .. 100
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub limit: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub since_id: Option<Id<GalleryPost>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub until_id: Option<Id<GalleryPost>>,
}

impl misskey_core::Request for Request {
    type Response = Vec<GalleryPost>;
    const ENDPOINT: &'static str = "gallery/posts";
}

impl_pagination!(Request, GalleryPost);

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let client = TestClient::new();
        client.user.test(Request::default()).await;
    }

    #[tokio::test]
    async fn request_with_limit() {
        let client = TestClient::new();

        client
            .test(Request {
                limit: Some(100),
                since_id: None,
                until_id: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_paginate() {
        let client = TestClient::new();
        let url = client.avatar_url().await;
        let file = client.upload_from_url(url).await;

        let post = client
            .test(crate::endpoint::gallery::posts::create::Request {
                title: "gallery post".to_string(),
                description: None,
                file_ids: vec![file.id],
                is_sensitive: None,
            })
            .await;

        client
            .test(Request {
                limit: None,
                since_id: Some(post.id.clone()),
                until_id: Some(post.id.clone()),
            })
            .await;
    }
}
