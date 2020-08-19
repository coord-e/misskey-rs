use crate::model::{
    antenna::AntennaId,
    note::{Note, NoteId},
};

use misskey_core::ApiRequest;
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub antenna_id: AntennaId,
    /// 1 .. 100
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since_id: Option<NoteId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until_id: Option<NoteId>,
}

impl ApiRequest for Request {
    type Response = Vec<Note>;
    const ENDPOINT: &'static str = "antennas/notes";
}
