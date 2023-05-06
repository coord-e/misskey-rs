use std::fmt::{self, Display};

use crate::model::{drive::DriveFile, id::Id, user::User};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Page {
    pub id: Id<Page>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub user_id: Id<User>,
    pub user: Box<User>,
    pub content: Content,
    pub variables: Variables,
    pub title: String,
    pub name: String,
    #[serde(default)]
    pub summary: Option<String>,
    pub align_center: bool,
    pub hide_title_when_pinned: bool,
    pub font: Font,
    #[cfg(feature = "12-31-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-31-0")))]
    pub script: String,
    pub eye_catching_image_id: Option<Id<DriveFile>>,
    pub eye_catching_image: Option<Box<DriveFile>>,
    pub attached_files: Vec<DriveFile>,
    pub liked_count: u64,
    #[serde(default)]
    pub is_liked: Option<bool>,
}

impl_entity!(Page);

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PageLike {
    pub id: Id<PageLike>,
    pub page: Page,
}

impl_entity!(PageLike);

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Content(Vec<serde_json::Map<String, serde_json::Value>>);

impl std::str::FromStr for Content {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Content, Self::Err> {
        serde_json::from_str(s)
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Variables(Vec<serde_json::Map<String, serde_json::Value>>);

impl std::str::FromStr for Variables {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Variables, Self::Err> {
        serde_json::from_str(s)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy)]
#[serde(rename_all = "kebab-case")]
pub enum Font {
    Serif,
    SansSerif,
}

impl Display for Font {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Font::Serif => f.write_str("serif"),
            Font::SansSerif => f.write_str("sans-serif"),
        }
    }
}

#[derive(Debug, Error, Clone)]
#[error("invalid font")]
pub struct ParseFontError {
    _priv: (),
}

impl std::str::FromStr for Font {
    type Err = ParseFontError;

    fn from_str(s: &str) -> Result<Font, Self::Err> {
        match s {
            "serif" | "Serif" => Ok(Font::Serif),
            "sans-serif" | "Sans-Serif" => Ok(Font::SansSerif),
            _ => Err(ParseFontError { _priv: () }),
        }
    }
}
