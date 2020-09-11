use std::collections::HashMap;

use crate::broker::{
    channel::{ChannelPongSender, ResponseSender, ResponseStreamSender},
    model::{BroadcastId, BrokerControl},
};
use crate::error::Result;
use crate::model::{
    incoming::{
        ApiMessage, ChannelMessage, ConnectedMessage, IncomingMessage, IncomingMessageType,
        NoteUpdatedMessage,
    },
    outgoing::OutgoingMessage,
    ApiRequestId, ChannelId,
};

use log::{info, warn};
use misskey_core::model::ApiResult;
use misskey_core::streaming::SubNoteId;
use serde_json::value::{self, Value};

#[derive(Debug)]
pub(crate) struct Handler {
    api: HashMap<ApiRequestId, ResponseSender<ApiResult<Value>>>,
    sub_note: HashMap<SubNoteId, ResponseStreamSender<Value>>,
    channel: HashMap<ChannelId, (ResponseStreamSender<Value>, Option<ChannelPongSender>)>,
    broadcast: HashMap<&'static str, HashMap<BroadcastId, ResponseStreamSender<Value>>>,
}

impl Handler {
    pub fn new() -> Handler {
        Handler {
            api: HashMap::new(),
            sub_note: HashMap::new(),
            channel: HashMap::new(),
            broadcast: HashMap::new(),
        }
    }

    pub fn control(&mut self, ctrl: BrokerControl) -> Option<OutgoingMessage> {
        match ctrl {
            BrokerControl::Api {
                id,
                endpoint,
                data,
                sender,
            } => {
                self.api.insert(id, sender);
                Some(OutgoingMessage::Api { id, endpoint, data })
            }
            BrokerControl::Connect {
                id,
                sender,
                params,
                name,
                pong,
            } => {
                self.channel.insert(id, (sender, Some(pong)));
                Some(OutgoingMessage::Connect {
                    channel: name,
                    id,
                    params,
                    pong: true,
                })
            }
            BrokerControl::Channel { id, message } => {
                Some(OutgoingMessage::Channel { id, message })
            }
            BrokerControl::Disconnect { id } => {
                self.channel.remove(&id);
                Some(OutgoingMessage::Disconnect { id })
            }
            BrokerControl::SubNote { id, sender } => {
                self.sub_note.insert(id.clone(), sender);
                Some(OutgoingMessage::SubNote { id })
            }
            BrokerControl::UnsubNote { id } => {
                self.sub_note.remove(&id);
                Some(OutgoingMessage::UnsubNote { id })
            }
            BrokerControl::StartBroadcast { id, type_, sender } => {
                self.broadcast
                    .entry(type_)
                    .or_insert_with(HashMap::new)
                    .insert(id, sender);
                None
            }
            BrokerControl::StopBroadcast { id } => {
                for senders in &mut self.broadcast.values_mut() {
                    if senders.remove(&id).is_some() {
                        break;
                    }
                }
                None
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        self.api.is_empty()
            && self.sub_note.is_empty()
            && self.channel.is_empty()
            && self.broadcast.values().all(|m| m.is_empty())
    }

    pub async fn handle(&mut self, msg: IncomingMessage) -> Result<()> {
        match msg.type_ {
            IncomingMessageType::Api(id) => {
                if let Some(sender) = self.api.remove(&id) {
                    let msg: ApiResult<ApiMessage> = value::from_value(msg.body)?;
                    sender.send(msg.map(Into::into));
                } else {
                    warn!("unknown API response message with id {}, skipping", id);
                    return Ok(());
                }
            }
            IncomingMessageType::Channel => {
                let ChannelMessage { id, message } = value::from_value(msg.body)?;

                let (sender, _) = match self.channel.get_mut(&id) {
                    Some(x) => x,
                    None => {
                        warn!("unhandled channel message with id {}, skipping", id);
                        return Ok(());
                    }
                };

                if sender.try_send(message).is_err() {
                    warn!("stale channel handler for id {}, deleted", id);
                    self.channel.remove(&id);
                }
            }
            IncomingMessageType::Connected => {
                let ConnectedMessage { id } = value::from_value(msg.body)?;

                let (_, pong) = match self.channel.get_mut(&id) {
                    Some(x) => x,
                    None => {
                        warn!("unhandled connected message with id {}, skipping", id);
                        return Ok(());
                    }
                };

                if let Some(pong) = pong.take() {
                    pong.send();
                } else {
                    warn!("duplicated connected message with id {}, skipping", id);
                }
            }
            IncomingMessageType::NoteUpdated => {
                let NoteUpdatedMessage { id, message } = value::from_value(msg.body)?;

                let sender = match self.sub_note.get_mut(&id) {
                    Some(x) => x,
                    None => {
                        warn!("unhandled subnote message with id {}, skipping", id);
                        return Ok(());
                    }
                };

                if sender.try_send(message).is_err() {
                    warn!("stale subnote handler for id {}, deleted", id);
                    self.sub_note.remove(&id);
                }
            }
            IncomingMessageType::Other(type_) => {
                // assuming other message types as broadcast

                let senders = match self.broadcast.get_mut(type_.as_str()) {
                    Some(x) => x,
                    None => {
                        info!("unhandled broadcast message {}, skipping", type_);
                        return Ok(());
                    }
                };

                let body = msg.body;
                senders.retain(|id, sender| {
                    if sender.try_send(body.clone()).is_err() {
                        warn!("stale broadcast handler {}:{}, deleted", type_, id);
                        false
                    } else {
                        true
                    }
                });
            }
        }

        Ok(())
    }
}
