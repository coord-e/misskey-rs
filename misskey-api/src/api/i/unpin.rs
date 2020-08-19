use crate::model::{note::NoteId, user::User};

use misskey_core::ApiRequest;
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub note_id: NoteId,
}

impl ApiRequest for Request {
    type Response = User;
    const ENDPOINT: &'static str = "i/unpin";
}
