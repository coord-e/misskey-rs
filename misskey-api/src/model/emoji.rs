use crate::model::id::Id;

use serde::{Deserialize, Serialize};
#[cfg(not(feature = "13-0-0"))]
use url::Url;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Emoji {
    pub id: Id<Emoji>,
    pub name: String,
    #[cfg(not(feature = "13-0-0"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "13-0-0"))))]
    pub url: Url,
    pub host: Option<String>,
    pub category: Option<String>,
    pub aliases: Vec<String>,
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
}
