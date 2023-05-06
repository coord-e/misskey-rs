use crate::model::{id::Id, page::Page, user::User};

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
    pub since_id: Option<Id<Page>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub until_id: Option<Id<Page>>,
}

impl misskey_core::Request for Request {
    type Response = Vec<Page>;
    const ENDPOINT: &'static str = "users/pages";
}

impl_pagination!(Request, Page);

#[cfg(test)]
mod tests {
    use ulid_crate::Ulid;

    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request_simple() {
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
        let page = client
            .user
            .test(
                crate::endpoint::pages::create::Request::builder()
                    .name(Ulid::new())
                    .build(),
            )
            .await;

        client
            .user
            .test(Request {
                user_id: user.id,
                limit: None,
                since_id: Some(page.id.clone()),
                until_id: Some(page.id.clone()),
            })
            .await;
    }
}
