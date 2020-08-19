use crate::model::emoji::EmojiId;

use misskey_core::ApiRequest;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub emoji_id: EmojiId,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub id: EmojiId,
}

impl ApiRequest for Request {
    type Response = Response;
    const ENDPOINT: &'static str = "admin/emoji/copy";
}
