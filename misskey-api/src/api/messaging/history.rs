use crate::model::messaging::MessagingMessage;

use misskey_core::ApiRequest;
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub group: bool,
    /// 1 .. 100
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u8>,
}

impl ApiRequest for Request {
    type Response = Vec<MessagingMessage>;
    const ENDPOINT: &'static str = "messaging/history";
}
