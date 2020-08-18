use crate::api::ApiRequest;
use crate::model::{
    file::FileId,
    page::PageId,
    user::{User, UserField},
};

use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birthday: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_id: Option<Option<FileId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub banner_id: Option<Option<FileId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<UserField>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_locked: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub careful_bot: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_accept_followed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_bot: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_cat: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_watch: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inject_featured_note: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub always_mark_nsfw: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pinned_page_id: Option<Option<PageId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub muted_words: Option<Vec<Vec<String>>>,
}

impl ApiRequest for Request {
    type Response = User;
    const ENDPOINT: &'static str = "i/update";
}
