use crate::model::{id::Id, log::ModerationLog};

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
    pub since_id: Option<Id<ModerationLog>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub until_id: Option<Id<ModerationLog>>,
}

impl misskey_core::Request for Request {
    type Response = Vec<ModerationLog>;
    const ENDPOINT: &'static str = "admin/show-moderation-logs";
}

impl_pagination!(Request, ModerationLog);

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
    async fn request_with_options() {
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
        let image_url = client.avatar_url().await;
        // this will create a moderation log entry
        client.admin.add_emoji_from_url(image_url).await;

        let logs = client
            .admin
            .test(Request {
                limit: None,
                since_id: None,
                until_id: None,
            })
            .await;

        client
            .admin
            .test(Request {
                limit: None,
                since_id: Some(logs[0].id.clone()),
                until_id: Some(logs[0].id.clone()),
            })
            .await;
    }
}
