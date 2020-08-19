use crate::model::user::User;

use misskey_core::ApiRequest;
use serde::Serialize;

pub mod favorites;
pub mod notifications;
pub mod pin;
pub mod read_all_messaging_messages;
pub mod read_all_unread_notes;
pub mod read_announcement;
pub mod unpin;
pub mod update;
pub mod user_group_invites;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {}

impl ApiRequest for Request {
    type Response = User;
    const ENDPOINT: &'static str = "i";
}
