use std::collections::HashMap;

use crate::broker::{
    channel::{ResponseSender, ResponseStreamSender},
    model::{BroadcastId, BrokerControl},
};
use crate::error::Result;
use crate::model::{
    message::{ApiMessage, Message, MessageType, OtherMessage},
    RequestId,
};

use log::{info, warn};
use misskey_core::model::ApiResult;
use serde_json::value::{self, Value};

#[derive(Debug)]
pub(crate) struct Handler {
    api: HashMap<RequestId, ResponseSender<ApiResult<Value>>>,
    subscription: HashMap<RequestId, ResponseStreamSender<Value>>,
    broadcast: HashMap<&'static str, HashMap<BroadcastId, ResponseStreamSender<Value>>>,
}

impl Handler {
    pub fn new() -> Handler {
        Handler {
            api: HashMap::new(),
            subscription: HashMap::new(),
            broadcast: HashMap::new(),
        }
    }

    pub fn update(&mut self, ctrl: BrokerControl) {
        match ctrl {
            BrokerControl::HandleApiResponse { id, sender } => {
                self.api.insert(id, sender);
            }
            // not using `type_` because we can determine corresponding sender by ID
            BrokerControl::Subscribe {
                id,
                sender,
                type_: _,
            } => {
                self.subscription.insert(id, sender);
            }
            BrokerControl::StartBroadcast { id, type_, sender } => {
                self.broadcast
                    .entry(type_)
                    .or_insert_with(HashMap::new)
                    .insert(id, sender);
            }
            BrokerControl::Unsubscribe { id } => {
                self.subscription.remove(&id);
            }
            BrokerControl::StopBroadcast { id } => {
                for senders in &mut self.broadcast.values_mut() {
                    if senders.remove(&id).is_some() {
                        break;
                    }
                }
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        self.api.is_empty()
            && self.subscription.is_empty()
            && self.broadcast.values().all(|m| m.is_empty())
    }

    pub async fn handle(&mut self, msg: Message) -> Result<()> {
        match msg.type_ {
            MessageType::Api(id) => {
                if let Some(sender) = self.api.remove(&id) {
                    let msg: ApiResult<ApiMessage> = value::from_value(msg.body)?;
                    sender.send(msg.map(Into::into));
                }
            }
            MessageType::Other(type_) => match value::from_value(msg.body)? {
                OtherMessage::WithId { id, content } => {
                    if let Some(sender) = self.subscription.get_mut(&id) {
                        if sender.try_send(content).is_err() {
                            warn!("stale subscription handler {:?}, deleted", id);
                            self.subscription.remove(&id);
                        }
                    }
                }
                OtherMessage::WithoutId(content) => {
                    let senders = match self.broadcast.get_mut(type_.as_str()) {
                        Some(x) => x,
                        None => {
                            info!("unhandled broadcast message {}, skipping", type_);
                            return Ok(());
                        }
                    };

                    senders.retain(|id, sender| {
                        if sender.try_send(content.clone()).is_err() {
                            warn!("stale broadcast handler {}:{}, deleted", type_, id);
                            false
                        } else {
                            true
                        }
                    });
                }
            },
        }

        Ok(())
    }
}
