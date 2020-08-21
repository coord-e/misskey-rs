use crate::model::announcement::AnnouncementId;

use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub announcement_id: AnnouncementId,
}

impl misskey_core::Request for Request {
    type Response = ();
    const ENDPOINT: &'static str = "i/read-announcement";
}

// TODO: tests
