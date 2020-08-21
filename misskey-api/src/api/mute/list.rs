use crate::model::muting::{Muting, MutingId};

use misskey_core::ApiRequest;
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    /// 1 .. 100
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since_id: Option<MutingId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until_id: Option<MutingId>,
}

impl ApiRequest for Request {
    type Response = Vec<Muting>;
    const ENDPOINT: &'static str = "mute/list";
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
            .test(crate::api::mute::create::Request {
                user_id: user.id.clone(),
            })
            .await;

        let mutings = client
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
                since_id: Some(mutings[0].id.clone()),
                until_id: Some(mutings[0].id.clone()),
            })
            .await;
    }
}