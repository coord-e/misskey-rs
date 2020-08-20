use std::collections::HashSet;

use crate::model::notification::{Notification, NotificationId, NotificationType};

use misskey_core::ApiRequest;
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    /// 1 .. 100
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since_id: Option<NotificationId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until_id: Option<NotificationId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub following: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mark_as_read: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_types: Option<HashSet<NotificationType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_types: Option<HashSet<NotificationType>>,
}

impl ApiRequest for Request {
    type Response = Vec<Notification>;
    const ENDPOINT: &'static str = "i/notifications";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let mut client = TestClient::new();
        client
            .test(Request {
                limit: None,
                since_id: None,
                until_id: None,
                following: None,
                mark_as_read: None,
                include_types: None,
                exclude_types: None,
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
                following: None,
                mark_as_read: None,
                include_types: None,
                exclude_types: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_with_options() {
        use crate::model::notification::NotificationType;

        let mut client = TestClient::new();
        client
            .test(Request {
                limit: None,
                since_id: None,
                until_id: None,
                following: Some(true),
                mark_as_read: Some(false),
                include_types: Some(
                    vec![NotificationType::Follow, NotificationType::Reply]
                        .into_iter()
                        .collect(),
                ),
                exclude_types: Some(vec![NotificationType::Mention].into_iter().collect()),
            })
            .await;
    }

    // TODO: request_paginate
}
