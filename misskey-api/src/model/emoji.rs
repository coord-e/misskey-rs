use crate::model::id::Id;

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
}
