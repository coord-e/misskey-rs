use crate::api::ApiRequest;
use crate::model::emoji::EmojiId;

use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub id: EmojiId,
}

impl ApiRequest for Request {
    type Response = ();
    const ENDPOINT: &'static str = "admin/emoji/remove";
}
