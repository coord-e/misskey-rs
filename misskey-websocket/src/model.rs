use derive_more::{Display, FromStr};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub(crate) mod message;
pub(crate) mod request;

// exported types
pub use message::{channel::MainStreamEvent, note_updated::NoteUpdateEvent};
pub use request::{ParseTimelineError, Timeline};

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, FromStr, Debug, Copy, Display)]
#[serde(transparent)]
pub(crate) struct ChannelId(pub Uuid);

impl ChannelId {
    pub fn new() -> Self {
        ChannelId(Uuid::new_v4())
    }
}
