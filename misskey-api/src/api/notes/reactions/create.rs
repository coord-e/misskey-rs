use crate::model::note::{NoteId, Reaction};

use misskey_core::ApiRequest;
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub note_id: NoteId,
    pub reaction: Reaction,
}

impl ApiRequest for Request {
    type Response = ();
    const ENDPOINT: &'static str = "notes/reactions/create";
}
