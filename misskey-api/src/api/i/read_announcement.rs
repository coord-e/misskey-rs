use crate::model::announcement::AnnouncementId;

use misskey_core::ApiRequest;
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub announcement_id: AnnouncementId,
}

impl ApiRequest for Request {
    type Response = ();
    const ENDPOINT: &'static str = "i/read-announcement";
}

// TODO: tests
