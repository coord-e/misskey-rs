use crate::api::ApiRequest;
use crate::model::note::NoteId;

use serde::Serialize;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub note_id: NoteId,
    pub choice: u64,
}

impl ApiRequest for Request {
    type Response = ();
    const ENDPOINT: &'static str = "notes/polls/vote";
}
