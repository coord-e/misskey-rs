use crate::model::{flash::FlashLike, id::Id};

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
    pub since_id: Option<Id<FlashLike>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub until_id: Option<Id<FlashLike>>,
}

impl misskey_core::Request for Request {
    type Response = Vec<FlashLike>;
    const ENDPOINT: &'static str = "flash/my-likes";
}

impl_pagination!(Request, FlashLike);

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let client = TestClient::new();
        let flash = client
            .admin
            .test(crate::endpoint::flash::create::Request::default())
            .await;
        client
            .user
            .test(crate::endpoint::flash::like::Request { flash_id: flash.id })
            .await;

        client.test(Request::default()).await;
    }

    #[tokio::test]
    async fn request_with_limit() {
        let client = TestClient::new();
        let flash = client
            .admin
            .test(crate::endpoint::flash::create::Request::default())
            .await;
        client
            .user
            .test(crate::endpoint::flash::like::Request { flash_id: flash.id })
            .await;

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
        let flash = client
            .admin
            .test(crate::endpoint::flash::create::Request::default())
            .await;
        client
            .user
            .test(crate::endpoint::flash::like::Request { flash_id: flash.id })
            .await;
        let likes = client.test(Request::default()).await;

        client
            .test(Request {
                limit: None,
                since_id: Some(likes[0].id),
                until_id: Some(likes[0].id),
            })
            .await;
    }
}
