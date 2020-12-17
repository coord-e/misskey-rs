use crate::model::{announcement::Announcement, id::Id};

use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AnnouncementWithReads {
    pub reads: u64,
    #[serde(flatten)]
    pub announcement: Announcement,
}

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
    pub since_id: Option<Id<Announcement>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub until_id: Option<Id<Announcement>>,
}

impl crate::PaginationItem for AnnouncementWithReads {
    type Id = Id<Announcement>;
    fn item_id(&self) -> Id<Announcement> {
        self.announcement.id
    }
}

impl misskey_core::Request for Request {
    type Response = Vec<AnnouncementWithReads>;
    const ENDPOINT: &'static str = "admin/announcements/list";
}

impl crate::PaginationRequest for Request {
    type Item = AnnouncementWithReads;

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
        let announcement = client
            .admin
            .test(crate::endpoint::admin::announcements::create::Request {
                title: "hello".to_string(),
                text: "ok".to_string(),
                image_url: None,
            })
            .await;

        client
            .admin
            .test(Request {
                limit: None,
                since_id: Some(announcement.id.clone()),
                until_id: Some(announcement.id.clone()),
            })
            .await;
    }
}
