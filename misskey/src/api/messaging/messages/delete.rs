use crate::api::ApiRequest;
use crate::model::messaging::MessagingMessageId;

use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub message_id: MessagingMessageId,
}

impl ApiRequest for Request {
    type Response = ();
    const ENDPOINT: &'static str = "messaging/messages/delete";
}
