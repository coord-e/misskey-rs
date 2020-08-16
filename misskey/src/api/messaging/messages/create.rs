use crate::api::ApiRequest;
use crate::model::{
    file::FileId, messaging::MessagingMessage, user::UserId, user_group::UserGroupId,
};

use serde::Serialize;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<UserId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<UserGroupId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_id: Option<FileId>,
}

impl ApiRequest for Request {
    type Response = MessagingMessage;
    const ENDPOINT: &'static str = "messaging/messages/create";
}
