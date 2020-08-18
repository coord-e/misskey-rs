use derivative::Derivative;
use derive_more::FromStr;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, FromStr, Derivative)]
#[serde(transparent)]
#[derivative(Debug = "transparent")]
pub struct EmojiId(pub String);

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Emoji {
    pub id: EmojiId,
    pub name: String,
    pub url: Url,
    pub host: Option<String>,
    pub category: Option<String>,
    pub aliases: Vec<String>,
}
