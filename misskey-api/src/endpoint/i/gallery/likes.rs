use crate::model::{gallery::GalleryLike, id::Id};

use serde::Serialize;
use typed_builder::TypedBuilder;

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
    pub since_id: Option<Id<GalleryLike>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub until_id: Option<Id<GalleryLike>>,
}

impl misskey_core::Request for Request {
    type Response = Vec<GalleryLike>;
    const ENDPOINT: &'static str = "i/gallery/likes";
}

impl_pagination!(Request, GalleryLike);

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
            .user
            .test(crate::endpoint::gallery::posts::create::Request {
                title: "gallery post".to_string(),
                description: None,
                file_ids: vec![file.id],
                is_sensitive: None,
            })
            .await;
        client
            .admin
            .test(crate::endpoint::gallery::posts::like::Request { post_id: post.id })
            .await;

        let likes = client
            .admin
            .test(Request {
                limit: None,
                since_id: None,
                until_id: None,
            })
            .await;

        client
            .admin
            .test(Request {
                limit: None,
                since_id: Some(likes[0].id.clone()),
                until_id: Some(likes[0].id.clone()),
            })
            .await;
    }
}
