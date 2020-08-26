use crate::model::RequestId;

use serde::Serialize;
use serde_json::value::Value;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ApiRequest {
    pub id: RequestId,
    pub endpoint: String,
    pub data: Value,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Request {
    #[serde(rename = "type")]
    pub type_: &'static str,
    pub body: Value,
}
