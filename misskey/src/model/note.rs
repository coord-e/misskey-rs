use std::collections::HashMap;

use crate::model::{
    file::{DriveFile, FileId},
    user::{User, UserId},
};

use chrono::{DateTime, Utc};
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Derivative)]
#[serde(transparent)]
#[derivative(Debug = "transparent")]
pub struct NoteId(pub String);

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Derivative)]
#[serde(transparent)]
#[derivative(Debug = "transparent")]
pub struct Tag(pub String);

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Derivative)]
#[serde(transparent)]
#[derivative(Debug = "transparent")]
pub struct ReactionType(pub String);

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub enum Visibility {
    Public,
    Home,
    Followers,
    Specified,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PollChoice {
    pub is_voted: bool,
    pub text: String,
    pub votes: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Poll {
    pub choices: Vec<PollChoice>,
    pub multiple: bool,
    pub expires_at: Option<DateTime<Utc>>,
    // pub expired_after: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Emoji {
    pub name: String,
    pub url: Url,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Note {
    pub id: NoteId,
    pub created_at: DateTime<Utc>,
    pub text: Option<String>,
    #[serde(default)]
    pub cw: Option<String>,
    pub user_id: UserId,
    pub user: User,
    #[serde(default)]
    pub reply_id: Option<NoteId>,
    #[serde(default)]
    pub renote_id: Option<NoteId>,
    #[serde(default)]
    pub reply: Option<Box<Note>>,
    #[serde(default)]
    pub renote: Option<Box<Note>>,
    #[serde(default = "default_false")]
    pub via_mobile: bool,
    #[serde(default = "default_false")]
    pub is_hidden: bool,
    pub visibility: Visibility,
    #[serde(default)]
    pub mentions: Vec<UserId>,
    #[serde(default)]
    pub visible_user_ids: Vec<UserId>,
    #[serde(default)]
    pub file_ids: Vec<FileId>,
    #[serde(default)]
    pub files: Vec<DriveFile>,
    #[serde(default)]
    pub tags: Vec<Tag>,
    #[serde(default)]
    pub poll: Option<Poll>,
    #[serde(default)]
    pub reactions: HashMap<ReactionType, u64>,
    #[serde(default)]
    pub emojis: Vec<Emoji>,
    pub renote_count: u64,
    pub replies_count: u64,
}

fn default_false() -> bool {
    false
}
