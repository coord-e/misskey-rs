use crate::model::{gallery::GalleryPost, id::Id, user::User};

use serde::Serialize;
use typed_builder::TypedBuilder;

#[derive(Serialize, Debug, Clone, TypedBuilder)]
#[serde(rename_all = "camelCase")]
#[builder(doc)]
pub struct Request {
    pub user_id: Id<User>,
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
    const ENDPOINT: &'static str = "users/gallery/posts";
}

impl_pagination!(Request, GalleryPost);

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let client = TestClient::new();
        let user = client.user.me().await;
        client
            .user
            .test(Request {
                user_id: user.id,
                limit: None,
                since_id: None,
                until_id: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_with_limit() {
        let client = TestClient::new();
        let user = client.user.me().await;

        client
            .test(Request {
                user_id: user.id,
                limit: Some(100),
                since_id: None,
                until_id: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_paginate() {
        let client = TestClient::new();
        let user = client.user.me().await;
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
                user_id: user.id,
                limit: None,
                since_id: Some(post.id.clone()),
                until_id: Some(post.id.clone()),
            })
            .await;
    }
}
