use crate::model::announcement::{Announcement, AnnouncementId};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    #[cfg(feature = "12-5-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-5-0")))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_unreads: Option<bool>,
    /// 1 .. 100
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since_id: Option<AnnouncementId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until_id: Option<AnnouncementId>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub is_read: bool,
    #[serde(flatten)]
    pub announcement: Announcement,
}

impl misskey_core::Request for Request {
    type Response = Vec<Announcement>;
    const ENDPOINT: &'static str = "announcements";
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
                #[cfg(feature = "12-5-0")]
                with_unreads: None,
                limit: None,
                since_id: None,
                until_id: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_with_options() {
        let mut client = TestClient::new();
        client
            .test(Request {
                #[cfg(feature = "12-5-0")]
                with_unreads: Some(true),
                limit: Some(100),
                since_id: None,
                until_id: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_paginate() {
        let mut client = TestClient::new();
        let announcement = client
            .admin
            .test(crate::api::admin::announcements::create::Request {
                title: "hello".to_string(),
                text: "ok".to_string(),
                image_url: None,
            })
            .await;

        client
            .test(Request {
                #[cfg(feature = "12-5-0")]
                with_unreads: None,
                limit: None,
                since_id: Some(announcement.id.clone()),
                until_id: Some(announcement.id.clone()),
            })
            .await;
    }
}
