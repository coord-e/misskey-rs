use std::sync::Arc;

use crate::broker::channel::{ResponseSender, ResponseStreamSender};
use crate::error::Error;
use crate::model::{
    message::{channel::MainStreamEvent, note_updated::NoteUpdateEvent},
    ChannelId,
};

use async_std::sync::RwLock;
use misskey::model::note::{Note, NoteId};
use serde_json::Value;

#[derive(Debug)]
pub enum BrokerControl {
    HandleApiResponse {
        id: ChannelId,
        sender: ResponseSender<Value>,
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
pub enum BrokerState {
    Working,
    Dead(Error),
}

impl BrokerState {
    pub fn dead(&self) -> Option<&Error> {
        match self {
            BrokerState::Working => None,
            BrokerState::Dead(e) => Some(e),
        }
    }
}

pub type SharedBrokerState = Arc<RwLock<BrokerState>>;
