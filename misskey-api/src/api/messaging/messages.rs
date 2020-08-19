use crate::model::{
    messaging::{MessagingMessage, MessagingMessageId},
    user::UserId,
    user_group::UserGroupId,
};

use misskey_core::ApiRequest;
use serde::Serialize;

pub mod create;
pub mod delete;
pub mod read;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mark_as_read: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<UserId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<UserGroupId>,
    /// 1 .. 100
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since_id: Option<MessagingMessageId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until_id: Option<MessagingMessageId>,
}

impl ApiRequest for Request {
    type Response = Vec<MessagingMessage>;
    const ENDPOINT: &'static str = "messaging/messages";
}
