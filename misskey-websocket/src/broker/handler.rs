use std::collections::HashMap;

use crate::broker::{
    channel::{ResponseSender, ResponseStreamSender},
    model::BrokerControl,
};
use crate::error::Result;
use crate::model::{
    message::{
        api::ApiMessage,
        channel::{ChannelMessage, MainStreamEvent, NotePostedMessage},
        note_updated::{NoteUpdateEvent, NoteUpdatedMessage},
        Message, MessageType,
    },
    ChannelId,
};

use log::{debug, warn};
use misskey::model::note::{Note, NoteId};
use serde_json::value::{self, Value};

pub struct Handler {
    api: HashMap<ChannelId, ResponseSender<Value>>,
    main_stream: HashMap<ChannelId, ResponseStreamSender<MainStreamEvent>>,
    timeline: HashMap<ChannelId, ResponseStreamSender<Note>>,
    note: HashMap<NoteId, ResponseStreamSender<NoteUpdateEvent>>,
}

impl Handler {
    pub fn new() -> Handler {
        Handler {
            api: HashMap::new(),
            main_stream: HashMap::new(),
            timeline: HashMap::new(),
            note: HashMap::new(),
        }
    }

    pub fn update(&mut self, ctrl: BrokerControl) {
        match ctrl {
            BrokerControl::HandleApiResponse { id, sender } => {
                self.api.insert(id, sender);
            }
            BrokerControl::SubscribeMainStream { id, sender } => {
                self.main_stream.insert(id, sender);
            }
            BrokerControl::SubscribeTimeline { id, sender } => {
                self.timeline.insert(id, sender);
            }
            BrokerControl::SubscribeNote { id, sender } => {
                self.note.insert(id, sender);
            }
            BrokerControl::UnsubscribeChannel(id) => {
                self.main_stream.remove(&id);
                self.timeline.remove(&id);
            }
            BrokerControl::UnsubscribeNote(id) => {
                self.note.remove(&id);
            }
        }
    }

    pub async fn handle(&mut self, msg: Message) -> Result<()> {
        debug!("received {:?} (broker)", msg);
        match msg.type_ {
            MessageType::Api(id) => {
                if let Some(sender) = self.api.remove(&id) {
                    let msg: ApiMessage = value::from_value(msg.body)?;
                    sender.send(msg.res);
                }
            }
            MessageType::Channel => match value::from_value(msg.body)? {
                ChannelMessage::MainStream { id, event } => {
                    if let Some(sender) = self.main_stream.get_mut(&id) {
                        if sender.try_send(event).is_err() {
                            warn!("stale main_stream handler {:?}, deleted", id);
                            self.main_stream.remove(&id);
                        }
                    }
                }
                ChannelMessage::Timeline {
                    id,
                    note_posted: NotePostedMessage::Note { note },
                } => {
                    if let Some(sender) = self.timeline.get_mut(&id) {
                        if sender.try_send(note).is_err() {
                            warn!("stale timeline handler {:?}, deleted", id);
                            self.timeline.remove(&id);
                        }
                    }
                }
            },
            MessageType::NoteUpdated => {
                let msg: NoteUpdatedMessage = value::from_value(msg.body)?;
                if let Some(sender) = self.note.get_mut(&msg.id) {
                    if sender.try_send(msg.event).is_err() {
                        warn!("stale note handler {:?}, deleted", msg.id);
                        self.note.remove(&msg.id);
                    }
                }
            }
        }

        Ok(())
    }
}
