use crate::model::{id::Id, page::PageLike};

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
    pub since_id: Option<Id<PageLike>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub until_id: Option<Id<PageLike>>,
}

impl misskey_core::Request for Request {
    type Response = Vec<PageLike>;
    const ENDPOINT: &'static str = "i/page_likes";
}

impl_pagination!(Request, PageLike);

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request_simple() {
        let client = TestClient::new();
        client
            .test(Request {
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
                limit: Some(100),
                since_id: None,
                until_id: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_paginate() {
        let client = TestClient::new();
        let page = client
            .admin
            .test(crate::endpoint::pages::create::Request::default())
            .await;
        client
            .user
            .test(crate::endpoint::pages::like::Request { page_id: page.id })
            .await;

        let likes = client
            .test(Request {
                limit: None,
                since_id: None,
                until_id: None,
            })
            .await;

        client
            .test(Request {
                limit: None,
                since_id: Some(likes[0].id.clone()),
                until_id: Some(likes[0].id.clone()),
            })
            .await;
    }
}
