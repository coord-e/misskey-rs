use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod message;
pub mod request;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ChannelId(pub Uuid);

impl ChannelId {
    pub fn new() -> Self {
        ChannelId(Uuid::new_v4())
    }
}
