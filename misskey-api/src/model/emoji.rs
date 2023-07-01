use crate::model::id::Id;
#[cfg(feature = "13-13-0")]
use crate::model::role::Role;

use serde::{Deserialize, Serialize};
#[cfg(any(not(feature = "13-0-0"), feature = "13-1-1"))]
use url::Url;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Emoji {
    pub id: Id<Emoji>,
    pub name: String,
    #[cfg(any(not(feature = "13-0-0"), feature = "13-7-0"))]
    #[cfg_attr(docsrs, doc(cfg(any(not(feature = "13-0-0"), feature = "13-7-0"))))]
    pub url: Url,
    pub host: Option<String>,
    pub category: Option<String>,
    pub aliases: Vec<String>,
    #[cfg(feature = "13-10-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "13-10-0")))]
    pub license: Option<String>,
    #[cfg(feature = "13-13-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "13-13-0")))]
    pub is_sensitive: Option<bool>,
    #[cfg(feature = "13-13-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "13-13-0")))]
    pub local_only: Option<bool>,
    #[cfg(feature = "13-13-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "13-13-0")))]
    pub role_ids_that_can_be_used_this_emoji_as_reaction: Option<Vec<Id<Role>>>,
}

impl_entity!(Emoji);

#[cfg(feature = "13-0-0")]
#[cfg_attr(docsrs, doc(cfg(feature = "13-0-0")))]
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EmojiSimple {
    pub name: String,
    pub aliases: Vec<String>,
    pub category: Option<String>,
    #[cfg(feature = "13-1-1")]
    #[cfg_attr(docsrs, doc(cfg(feature = "13-1-1")))]
    pub url: Url,
    #[cfg(feature = "13-13-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "13-13-0")))]
    pub is_sensitive: Option<bool>,
    #[cfg(feature = "13-13-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "13-13-0")))]
    pub role_ids_that_can_be_used_this_emoji_as_reaction: Option<Vec<Id<Role>>>,
}
