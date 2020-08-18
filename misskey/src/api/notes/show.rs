use crate::api::ApiRequest;
use crate::model::note::{Note, NoteId};

use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub note_id: NoteId,
}

impl ApiRequest for Request {
    type Response = Note;
    const ENDPOINT: &'static str = "notes/show";
}
