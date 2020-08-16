use crate::api::ApiRequest;
use crate::model::note::{Note, NoteId, Tag};

use serde::Serialize;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub tag: Tag,
    pub query: Vec<Vec<String>>,
    pub reply: Option<bool>,
    pub renote: Option<bool>,
    pub poll: Option<bool>,
    pub with_files: bool,
    /// 1 .. 100, default: 10
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since_id: Option<NoteId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until_id: Option<NoteId>,
}

impl ApiRequest for Request {
    type Response = Vec<Note>;
    const ENDPOINT: &'static str = "notes/search-by-tag";
}
