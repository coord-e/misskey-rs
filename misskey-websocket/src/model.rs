use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod incoming;
pub mod outgoing;

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Debug)]
#[serde(transparent)]
pub struct SubNoteId(pub String);

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[serde(transparent)]
pub struct ChannelId(pub Uuid);

impl ChannelId {
    pub fn uuid() -> Self {
        ChannelId(Uuid::new_v4())
    }
}

#[cfg(not(feature = "12-111-0"))]
#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[serde(transparent)]
pub struct ApiRequestId(pub Uuid);

#[cfg(not(feature = "12-111-0"))]
impl ApiRequestId {
    pub fn uuid() -> Self {
        ApiRequestId(Uuid::new_v4())
    }
}
