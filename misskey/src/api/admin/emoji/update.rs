use crate::api::ApiRequest;
use crate::model::emoji::EmojiId;

use serde::Serialize;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub id: EmojiId,
    pub name: String,
    pub category: Option<String>,
    pub aliases: Vec<String>,
}

impl ApiRequest for Request {
    type Response = ();
    const ENDPOINT: &'static str = "admin/emoji/update";
}
