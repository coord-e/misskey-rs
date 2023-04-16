use std::collections::HashMap;

#[cfg(not(feature = "12-111-0"))]
use crate::broker::channel::ResponseSender;
use crate::broker::{
    channel::{ChannelPongSender, ResponseStreamSender},
    model::{BroadcastId, BrokerControl},
};
use crate::error::Result;
#[cfg(not(feature = "12-111-0"))]
use crate::model::{incoming::ApiMessage, ApiRequestId};
use crate::model::{
    incoming::{
        ChannelMessage, ConnectedMessage, IncomingMessage, IncomingMessageType, NoteUpdatedMessage,
    },
    outgoing::OutgoingMessage,
    ChannelId, SubNoteId,
};

use log::{info, warn};
#[cfg(not(feature = "12-111-0"))]
use misskey_core::model::ApiResult;
use serde_json::value::{self, Value};

#[cfg(not(feature = "12-111-0"))]
#[derive(Debug)]
struct ApiHandler {
    message: OutgoingMessage,
    sender: ResponseSender<ApiResult<Value>>,
}

#[derive(Debug)]
struct SubNoteHandler {
    message: OutgoingMessage,
    sender: ResponseStreamSender<Value>,
}

#[derive(Debug)]
struct ChannelHandler {
    message: OutgoingMessage,
    pong: Option<ChannelPongSender>,
    sender: ResponseStreamSender<Value>,
}

#[derive(Debug)]
pub(crate) struct Handler {
    #[cfg(not(feature = "12-111-0"))]
    api: HashMap<ApiRequestId, ApiHandler>,
    sub_note: HashMap<SubNoteId, SubNoteHandler>,
    channel: HashMap<ChannelId, ChannelHandler>,
    broadcast: HashMap<&'static str, HashMap<BroadcastId, ResponseStreamSender<Value>>>,
}

impl Handler {
    pub fn new() -> Handler {
        Handler {
            #[cfg(not(feature = "12-111-0"))]
            api: HashMap::new(),
            sub_note: HashMap::new(),
            channel: HashMap::new(),
            broadcast: HashMap::new(),
        }
    }

    pub fn restore_messages(&mut self) -> Vec<OutgoingMessage> {
        let mut messages = Vec::new();

        #[cfg(not(feature = "12-111-0"))]
        for ApiHandler { message, .. } in self.api.values() {
            messages.push(message.clone());
        }

        for SubNoteHandler { message, .. } in self.sub_note.values() {
            messages.push(message.clone());
        }

        for ChannelHandler { message, .. } in self.channel.values() {
            // TODO: Handle ping/pong in reconnect
            messages.push(message.clone());
        }

        messages
    }

    pub fn control(&mut self, ctrl: BrokerControl) -> Option<OutgoingMessage> {
        match ctrl {
            #[cfg(not(feature = "12-111-0"))]
            BrokerControl::Api {
                id,
                endpoint,
                data,
                sender,
            } => {
                let message = OutgoingMessage::Api { id, endpoint, data };
                let handler = ApiHandler {
                    message: message.clone(),
                    sender,
                };
                self.api.insert(id, handler);
                Some(message)
            }
            BrokerControl::Connect {
                id,
                sender,
                params,
                name,
                pong,
            } => {
                let message = OutgoingMessage::Connect {
                    channel: name,
                    id,
                    params,
                    pong: true,
                };
                let handler = ChannelHandler {
                    message: message.clone(),
                    sender,
                    pong: Some(pong),
                };
                self.channel.insert(id, handler);
                Some(message)
            }
            BrokerControl::Channel { id, message } => {
                Some(OutgoingMessage::Channel { id, message })
            }
            BrokerControl::Disconnect { id } => {
                self.channel.remove(&id);
                Some(OutgoingMessage::Disconnect { id })
            }
            BrokerControl::SubNote { id, sender } => {
                let message = OutgoingMessage::SubNote { id: id.clone() };
                let handler = SubNoteHandler {
                    message: message.clone(),
                    sender,
                };
                self.sub_note.insert(id, handler);
                Some(message)
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

    #[cfg(not(feature = "12-111-0"))]
    pub fn is_empty(&self) -> bool {
        self.api.is_empty()
            && self.sub_note.is_empty()
            && self.channel.is_empty()
            && self.broadcast.values().all(|m| m.is_empty())
    }

    #[cfg(feature = "12-111-0")]
    pub fn is_empty(&self) -> bool {
        self.sub_note.is_empty()
            && self.channel.is_empty()
            && self.broadcast.values().all(|m| m.is_empty())
    }

    pub async fn handle(&mut self, msg: IncomingMessage) -> Result<()> {
        match msg.type_ {
            #[cfg(not(feature = "12-111-0"))]
            IncomingMessageType::Api(id) => {
                if let Some(ApiHandler { sender, .. }) = self.api.remove(&id) {
                    let msg: ApiResult<ApiMessage> = value::from_value(msg.body)?;
                    sender.send(msg.map(|m| m.res));
                } else {
                    warn!("unknown API response message with {:?}, skipping", id);
                    return Ok(());
                }
            }
            IncomingMessageType::Channel => {
                let ChannelMessage { id, message } = value::from_value(msg.body)?;

                let ChannelHandler { sender, .. } = match self.channel.get_mut(&id) {
                    Some(x) => x,
                    None => {
                        warn!("unhandled channel message with {:?}, skipping", id);
                        return Ok(());
                    }
                };

                if sender.try_send(message).is_err() {
                    warn!("stale channel handler for {:?}, deleted", id);
                    self.channel.remove(&id);
                }
            }
            IncomingMessageType::Connected => {
                let ConnectedMessage { id } = value::from_value(msg.body)?;

                let ChannelHandler { pong, .. } = match self.channel.get_mut(&id) {
                    Some(x) => x,
                    None => {
                        warn!("unhandled connected message with {:?}, skipping", id);
                        return Ok(());
                    }
                };

                if let Some(pong) = pong.take() {
                    pong.send();
                } else {
                    info!("duplicated connected message with {:?}, skipping", id);
                }
            }
            IncomingMessageType::NoteUpdated => {
                let NoteUpdatedMessage { id, message } = value::from_value(msg.body)?;

                let SubNoteHandler { sender, .. } = match self.sub_note.get_mut(&id) {
                    Some(x) => x,
                    None => {
                        warn!("unhandled subnote message with {:?}, skipping", id);
                        return Ok(());
                    }
                };

                if sender.try_send(message).is_err() {
                    warn!("stale subnote handler for {:?}, deleted", id);
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
                        warn!("stale broadcast handler {}:{:?}, deleted", type_, id);
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
