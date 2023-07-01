use crate::model::{id::Id, renote_muting::RenoteMuting};

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
    pub since_id: Option<Id<RenoteMuting>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub until_id: Option<Id<RenoteMuting>>,
}

impl misskey_core::Request for Request {
    type Response = Vec<RenoteMuting>;
    const ENDPOINT: &'static str = "renote-mute/list";
}

impl_pagination!(Request, RenoteMuting);

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
        let (user, _) = client.admin.create_user().await;

        client
            .user
            .test(crate::endpoint::renote_mute::create::Request { user_id: user.id })
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
