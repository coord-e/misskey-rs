use crate::model::{ad::Ad, id::Id};

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
    pub since_id: Option<Id<Ad>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub until_id: Option<Id<Ad>>,
}

impl misskey_core::Request for Request {
    type Response = Vec<Ad>;
    const ENDPOINT: &'static str = "admin/ad/list";
}

impl_pagination!(Request, Ad);

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
    async fn request_with_limit() {
        let client = TestClient::new();

        client
            .admin
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
        client
            .admin
            .test(
                crate::endpoint::admin::ad::create::Request::builder()
                    .url(url.clone())
                    .image_url(url.clone())
                    .expires_at(chrono::Utc::now() + chrono::Duration::hours(1))
                    .build(),
            )
            .await;

        let ads = client.admin.test(Request::default()).await;

        client
            .admin
            .test(Request {
                limit: None,
                since_id: Some(ads[0].id.clone()),
                until_id: Some(ads[0].id.clone()),
            })
            .await;
    }
}
