use std::collections::HashSet;

use crate::model::notification::{Notification, NotificationId, NotificationType};

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
    pub since_id: Option<NotificationId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub until_id: Option<NotificationId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub following: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub mark_as_read: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub include_types: Option<HashSet<NotificationType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub exclude_types: Option<HashSet<NotificationType>>,
}

impl misskey_core::Request for Request {
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
        client.test(Request::default()).await;
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

    #[tokio::test]
    #[cfg(feature = "12-27-0")]
    async fn request_paginate() {
        let mut client = TestClient::new();
        client
            .test(crate::api::notifications::create::Request {
                body: "hi".to_string(),
                header: None,
                icon: None,
            })
            .await;

        let notification = client
            .test(Request {
                limit: None,
                since_id: None,
                until_id: None,
                following: None,
                mark_as_read: None,
                include_types: None,
                exclude_types: None,
            })
            .await
            .pop()
            .unwrap();

        client
            .test(Request {
                limit: None,
                since_id: Some(notification.id.clone()),
                until_id: Some(notification.id.clone()),
                following: None,
                mark_as_read: None,
                include_types: None,
                exclude_types: None,
            })
            .await;
    }
}
