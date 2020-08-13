use crate::api::ApiRequest;
use crate::model::note::Note;

use serde::Serialize;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub offset: u64,
    /// 1 .. 100, default: 10
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u8>,
}

impl ApiRequest for Request {
    type Response = Vec<Note>;
    const ENDPOINT: &'static str = "notes/featured";
}
