use derive_more::FromStr;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod message;
pub mod request;

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, FromStr, Debug, Copy)]
#[serde(transparent)]
pub struct ChannelId(pub Uuid);

impl ChannelId {
    pub fn new() -> Self {
        ChannelId(Uuid::new_v4())
    }
}
