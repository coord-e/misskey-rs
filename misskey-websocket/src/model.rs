use derive_more::{Display, FromStr};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod incoming;
pub mod outgoing;

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash, FromStr, Debug, Display)]
#[serde(transparent)]
pub struct ChannelId(pub Uuid);

impl ChannelId {
    pub fn uuid() -> Self {
        ChannelId(Uuid::new_v4())
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, FromStr, Debug, Display)]
#[serde(transparent)]
pub struct ApiRequestId(pub Uuid);

impl ApiRequestId {
    pub fn uuid() -> Self {
        ApiRequestId(Uuid::new_v4())
    }
}
