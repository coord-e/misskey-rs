use crate::api::ApiRequest;
use crate::model::{note::NoteId, user::User};

use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub note_id: NoteId,
}

impl ApiRequest for Request {
    type Response = User;
    const ENDPOINT: &'static str = "i/pin";
}
