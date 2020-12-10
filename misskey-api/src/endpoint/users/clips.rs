use crate::model::{clip::Clip, id::Id, user::User};

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
    pub since_id: Option<Id<Clip>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub until_id: Option<Id<Clip>>,
}

impl misskey_core::Request for Request {
    type Response = Vec<Clip>;
    const ENDPOINT: &'static str = "users/clips";
}

impl_pagination!(Request, Clip);

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request_simple() {
        let client = TestClient::new();
        let user = client.user.me().await;
        client
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
            .user
            .test(crate::endpoint::clips::create::Request {
                name: "test".to_string(),
                is_public: None,
                description: None,
            })
            .await;

        client
            .user
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
        let clip = client
            .user
            .test(crate::endpoint::clips::create::Request {
                name: "test".to_string(),
                is_public: None,
                description: None,
            })
            .await;

        client
            .user
            .test(Request {
                user_id: user.id,
                limit: None,
                since_id: Some(clip.id),
                until_id: Some(clip.id),
            })
            .await;
    }
}
