use std::sync::Arc;

use crate::broker::channel::{ResponseSender, ResponseStreamSender};
use crate::error::Error;
use crate::model::{
    message::{channel::MainStreamEvent, note_updated::NoteUpdateEvent},
    ChannelId,
};

use async_std::sync::RwLock;
use misskey_api::model::note::{Note, NoteId};
use misskey_core::model::ApiResult;
use serde_json::Value;

#[derive(Debug)]
pub(crate) enum BrokerControl {
    HandleApiResponse {
        id: ChannelId,
        sender: ResponseSender<ApiResult<Value>>,
    },
    SubscribeMainStream {
        id: ChannelId,
        sender: ResponseStreamSender<MainStreamEvent>,
    },
    SubscribeTimeline {
        id: ChannelId,
        sender: ResponseStreamSender<Note>,
    },
    SubscribeNote {
        id: NoteId,
        sender: ResponseStreamSender<NoteUpdateEvent>,
    },
    UnsubscribeChannel(ChannelId),
    UnsubscribeNote(NoteId),
}

#[derive(Debug, Clone)]
pub(crate) enum BrokerState {
    Working,
    Exited,
    Dead(Error),
}

impl BrokerState {
    pub fn dead(&self) -> Option<&Error> {
        match self {
            BrokerState::Working => None,
            // TODO: clearify the guarantee that no one asks for `BrokerState` after broker is dead
            BrokerState::Exited => panic!("asked if broker is dead while it is already exited"),
            BrokerState::Dead(e) => Some(e),
        }
    }
}

pub(crate) type SharedBrokerState = Arc<RwLock<BrokerState>>;
