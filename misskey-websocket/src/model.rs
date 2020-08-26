use derive_more::{Display, FromStr};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub(crate) mod message;
pub(crate) mod request;

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, FromStr, Debug, Display)]
#[serde(transparent)]
pub(crate) struct RequestId(pub String);

impl RequestId {
    pub fn uuid() -> Self {
        RequestId(Uuid::new_v4().to_string())
    }
}
