use crate::api::ApiRequest;
use crate::model::{
    note::{Note, NoteId},
    user_list::UserListId,
};

use chrono::{serde::ts_milliseconds_option, DateTime, Utc};
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub list_id: UserListId,
    pub with_files: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_my_renotes: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_renoted_my_notes: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_local_renotes: Option<bool>,
    /// 1 .. 100, default: 10
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since_id: Option<NoteId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until_id: Option<NoteId>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        with = "ts_milliseconds_option"
    )]
    pub since_date: Option<DateTime<Utc>>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        with = "ts_milliseconds_option"
    )]
    pub until_date: Option<DateTime<Utc>>,
}

impl ApiRequest for Request {
    type Response = Vec<Note>;
    const ENDPOINT: &'static str = "notes/user-list-timeline";
}
