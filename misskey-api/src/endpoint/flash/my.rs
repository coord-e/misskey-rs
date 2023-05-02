use crate::model::{flash::Flash, id::Id};

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
    pub since_id: Option<Id<Flash>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub until_id: Option<Id<Flash>>,
}

impl misskey_core::Request for Request {
    type Response = Vec<Flash>;
    const ENDPOINT: &'static str = "flash/my";
}

impl_pagination!(Request, Flash);

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let client = TestClient::new();
        client
            .test(crate::endpoint::flash::create::Request::default())
            .await;

        client.test(Request::default()).await;
    }

    #[tokio::test]
    async fn request_with_limit() {
        let client = TestClient::new();
        client
            .test(crate::endpoint::flash::create::Request::default())
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
            .test(crate::endpoint::flash::create::Request::default())
            .await;

        client
            .test(Request {
                limit: None,
                since_id: Some(flash.id),
                until_id: Some(flash.id),
            })
            .await;
    }
}
