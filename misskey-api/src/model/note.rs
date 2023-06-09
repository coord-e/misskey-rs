use std::collections::HashMap;
use std::fmt::{self, Display};
use std::str::FromStr;

use crate::model::{channel::Channel, drive::DriveFile, id::Id, user::User};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use url::Url;

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Debug)]
#[serde(transparent)]
pub struct Tag(pub String);

impl Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use std::fmt::Write;
        if !self.0.starts_with('#') {
            f.write_char('#')?;
        }
        Display::fmt(&self.0, f)
    }
}

impl FromStr for Tag {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Tag, Self::Err> {
        Ok(Tag(s.to_string()))
    }
}

impl<S: Into<String>> From<S> for Tag {
    fn from(s: S) -> Tag {
        Tag(s.into())
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Debug)]
#[serde(transparent)]
pub struct Reaction(pub String);

impl Display for Reaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl FromStr for Reaction {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Reaction, Self::Err> {
        Ok(Reaction(s.to_string()))
    }
}

impl<S: Into<String>> From<S> for Reaction {
    fn from(s: S) -> Reaction {
        Reaction(s.into())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Copy)]
#[serde(rename_all = "camelCase")]
pub enum Visibility {
    Public,
    Home,
    Followers,
    Specified,
}

#[derive(Debug, Error, Clone)]
#[error("invalid note visibility")]
pub struct ParseVisibilityError {
    _priv: (),
}

impl std::str::FromStr for Visibility {
    type Err = ParseVisibilityError;

    fn from_str(s: &str) -> Result<Visibility, Self::Err> {
        match s {
            "public" | "Public" => Ok(Visibility::Public),
            "home" | "Home" => Ok(Visibility::Home),
            "followers" | "Followers" => Ok(Visibility::Followers),
            "specified" | "Specified" => Ok(Visibility::Specified),
            _ => Err(ParseVisibilityError { _priv: () }),
        }
    }
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
}

// packed `Emoji` for `Note`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NoteEmoji {
    pub name: String,
    pub url: Url,
}

// packed `Channel` for `Note`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NoteChannel {
    pub id: Id<Channel>,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Note {
    pub id: Id<Note>,
    pub created_at: DateTime<Utc>,
    pub text: Option<String>,
    #[serde(default)]
    pub cw: Option<String>,
    pub user_id: Id<User>,
    pub user: User,
    #[serde(default)]
    pub reply_id: Option<Id<Note>>,
    #[serde(default)]
    pub renote_id: Option<Id<Note>>,
    #[serde(default)]
    pub reply: Option<Box<Note>>,
    #[serde(default)]
    pub renote: Option<Box<Note>>,
    #[cfg(not(feature = "12-96-0"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "12-96-0"))))]
    #[serde(default = "default_false")]
    pub via_mobile: bool,
    #[serde(default = "default_false")]
    pub is_hidden: bool,
    #[serde(default = "default_false")]
    pub local_only: bool,
    pub visibility: Visibility,
    #[serde(default)]
    pub mentions: Vec<Id<User>>,
    #[serde(default)]
    pub visible_user_ids: Vec<Id<User>>,
    pub file_ids: Vec<Id<DriveFile>>,
    pub files: Vec<DriveFile>,
    #[serde(default)]
    pub tags: Vec<Tag>,
    #[serde(default)]
    pub poll: Option<Poll>,
    pub reactions: HashMap<Reaction, u64>,
    pub emojis: Vec<NoteEmoji>,
    pub renote_count: u64,
    pub replies_count: u64,
    #[cfg(feature = "12-47-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-47-0")))]
    #[serde(default)]
    pub channel_id: Option<Id<Channel>>,
    #[cfg(feature = "12-47-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-47-0")))]
    #[serde(default)]
    pub channel: Option<NoteChannel>,
}

fn default_false() -> bool {
    false
}

impl_entity!(Note);
