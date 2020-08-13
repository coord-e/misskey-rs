use crate::api::ApiRequest;
use crate::model::note::{Note, NoteId};

use serde::Serialize;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    note_id: NoteId,
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
    const ENDPOINT: &'static str = "notes/children";
}
