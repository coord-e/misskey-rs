use crate::api::ApiRequest;
use crate::model::{emoji::EmojiId, file::FileId};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub file_id: FileId,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub id: EmojiId,
}

impl ApiRequest for Request {
    type Response = Response;
    const ENDPOINT: &'static str = "admin/emoji/add";
}
