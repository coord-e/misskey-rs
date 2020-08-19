use crate::model::muting::{Muting, MutingId};

use misskey_core::ApiRequest;
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    /// 1 .. 100
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since_id: Option<MutingId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until_id: Option<MutingId>,
}

impl ApiRequest for Request {
    type Response = Vec<Muting>;
    const ENDPOINT: &'static str = "mute/list";
}
