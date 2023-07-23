#[cfg(not(feature = "12-111-0"))]
use crate::model::ApiRequestId;
use crate::model::{ChannelId, SubNoteId};

use serde::Serialize;
use serde_json::Value;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase", tag = "type", content = "body")]
pub enum OutgoingMessage {
    #[cfg(not(feature = "12-111-0"))]
    Api {
        id: ApiRequestId,
        endpoint: &'static str,
        data: Value,
    },
    Connect {
        id: ChannelId,
        channel: &'static str,
        params: Value,
        pong: bool,
    },
    Channel {
        id: ChannelId,
        #[serde(flatten)]
        message: Value,
    },
    Disconnect {
        id: ChannelId,
    },
    SubNote {
        id: SubNoteId,
    },
    UnsubNote {
        id: SubNoteId,
    },
}
