use crate::model::{drive::DriveFileId, emoji::EmojiId};

use misskey_core::ApiRequest;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub file_id: DriveFileId,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub id: EmojiId,
}

impl ApiRequest for Request {
    type Response = Response;
    const ENDPOINT: &'static str = "admin/emoji/add";
}
