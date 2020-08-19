use crate::model::note::NoteId;

use misskey_core::ApiRequest;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub note_id: NoteId,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub is_favorited: bool,
    pub is_watching: bool,
}

impl ApiRequest for Request {
    type Response = Response;
    const ENDPOINT: &'static str = "notes/state";
}
