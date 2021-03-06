use crate::model::{announcement::Announcement, id::Id};

use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Serialize, Default, Debug, Clone, TypedBuilder)]
#[serde(rename_all = "camelCase")]
#[builder(doc)]
pub struct Request {
    #[cfg(feature = "12-5-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-5-0")))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub with_unreads: Option<bool>,
    /// 1 .. 100
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub limit: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub since_id: Option<Id<Announcement>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub until_id: Option<Id<Announcement>>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AnnouncementWithIsRead {
    pub is_read: bool,
    #[serde(flatten)]
    pub announcement: Announcement,
}

impl crate::PaginationItem for AnnouncementWithIsRead {
    type Id = Id<Announcement>;
    fn item_id(&self) -> Id<Announcement> {
        self.announcement.id
    }
}

impl misskey_core::Request for Request {
    type Response = Vec<AnnouncementWithIsRead>;
    const ENDPOINT: &'static str = "announcements";
}

impl crate::PaginationRequest for Request {
    type Item = AnnouncementWithIsRead;

    fn set_since_id(&mut self, id: Id<Announcement>) {
        self.since_id.replace(id);
    }

    fn set_until_id(&mut self, id: Id<Announcement>) {
        self.until_id.replace(id);
    }

    fn set_limit(&mut self, limit: u8) {
        self.limit.replace(limit);
    }
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let client = TestClient::new();
        client.test(Request::default()).await;
    }

    #[tokio::test]
    async fn request_with_options() {
        let client = TestClient::new();
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
        let client = TestClient::new();
        let announcement = client
            .admin
            .test(crate::endpoint::admin::announcements::create::Request {
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
