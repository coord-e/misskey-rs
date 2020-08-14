use crate::broker::channel::{ResponseSender, ResponseStreamSender};
use crate::model::{
    message::{channel::MainStreamEvent, note_updated::NoteUpdateEvent},
    ChannelId,
};

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
