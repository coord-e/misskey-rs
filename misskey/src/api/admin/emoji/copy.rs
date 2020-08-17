use crate::api::ApiRequest;
use crate::model::emoji::EmojiId;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub emoji_id: EmojiId,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub id: EmojiId,
}

impl ApiRequest for Request {
    type Response = Response;
    const ENDPOINT: &'static str = "admin/emoji/copy";
}
