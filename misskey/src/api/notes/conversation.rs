use crate::api::ApiRequest;
use crate::model::note::{Note, NoteId};

use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub note_id: NoteId,
    pub offset: i64,
    /// 1 .. 100, default: 10
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u8>,
}

impl ApiRequest for Request {
    type Response = Vec<Note>;
    const ENDPOINT: &'static str = "notes/conversation";
}
