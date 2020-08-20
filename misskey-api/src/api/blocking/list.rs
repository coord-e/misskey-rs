use crate::model::blocking::{Blocking, BlockingId};

use misskey_core::ApiRequest;
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    /// 1 .. 100
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since_id: Option<BlockingId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until_id: Option<BlockingId>,
}

impl ApiRequest for Request {
    type Response = Vec<Blocking>;
    const ENDPOINT: &'static str = "blocking/list";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let mut client = TestClient::new();

        client
            .user
            .test(Request {
                limit: None,
                since_id: None,
                until_id: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_with_limit() {
        let mut client = TestClient::new();

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
        let mut client = TestClient::new();
        let (user, _) = client.admin.create_user().await;

        client
            .user
            .test(crate::api::blocking::create::Request {
                user_id: user.id.clone(),
            })
            .await;

        let blockings = client
            .user
            .test(Request {
                limit: None,
                since_id: None,
                until_id: None,
            })
            .await;

        client
            .user
            .test(Request {
                limit: None,
                since_id: Some(blockings[0].id.clone()),
                until_id: Some(blockings[0].id.clone()),
            })
            .await;
    }
}
