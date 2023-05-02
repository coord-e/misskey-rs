use crate::model::following::FollowRequest;
#[cfg(feature = "13-0-0")]
use crate::model::id::Id;

use serde::Serialize;
use typed_builder::TypedBuilder;

#[derive(Serialize, Default, Debug, Clone, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    /// 1 .. 100
    #[cfg(feature = "13-0-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "13-0-0")))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub limit: Option<u8>,
    #[cfg(feature = "13-0-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "13-0-0")))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub since_id: Option<Id<FollowRequest>>,
    #[cfg(feature = "13-0-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "13-0-0")))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub until_id: Option<Id<FollowRequest>>,
}

impl misskey_core::Request for Request {
    type Response = Vec<FollowRequest>;
    const ENDPOINT: &'static str = "following/requests/list";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let client = TestClient::new();
        let (new_user, new_client) = client.admin.create_user().await;

        new_client
            .test(
                crate::endpoint::i::update::Request::builder()
                    .is_locked(true)
                    .build(),
            )
            .await;
        client
            .user
            .test(crate::endpoint::following::create::Request {
                user_id: new_user.id,
            })
            .await;

        new_client.test(Request::default()).await;
    }

    #[cfg(feature = "13-0-0")]
    #[tokio::test]
    async fn request_with_limit() {
        let client = TestClient::new();
        let (new_user, new_client) = client.admin.create_user().await;

        new_client
            .test(
                crate::endpoint::i::update::Request::builder()
                    .is_locked(true)
                    .build(),
            )
            .await;
        client
            .user
            .test(crate::endpoint::following::create::Request {
                user_id: new_user.id,
            })
            .await;

        client
            .test(Request {
                limit: Some(100),
                since_id: None,
                until_id: None,
            })
            .await;
    }

    #[cfg(feature = "13-0-0")]
    #[tokio::test]
    async fn request_paginate() {
        let client = TestClient::new();
        let (new_user, new_client) = client.admin.create_user().await;

        new_client
            .test(
                crate::endpoint::i::update::Request::builder()
                    .is_locked(true)
                    .build(),
            )
            .await;
        client
            .user
            .test(crate::endpoint::following::create::Request {
                user_id: new_user.id,
            })
            .await;
        let requests = new_client.test(Request::default()).await;

        new_client
            .test(Request {
                limit: None,
                since_id: Some(requests[0].id.clone()),
                until_id: Some(requests[0].id.clone()),
            })
            .await;
    }
}
