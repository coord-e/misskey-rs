use derive_more::{Display, FromStr};
use serde::{Deserialize, Serialize};
use serde_json::value::Value;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, FromStr, Debug, Display)]
#[serde(transparent)]
pub(crate) struct ApiRequestId(pub Uuid);

impl ApiRequestId {
    pub fn uuid() -> Self {
        ApiRequestId(Uuid::new_v4())
    }
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ApiRequest {
    pub id: ApiRequestId,
    pub endpoint: String,
    pub data: Value,
}

impl misskey_core::streaming::Request for ApiRequest {
    const TYPE: &'static str = "api";
}
