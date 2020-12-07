use crate::model::id::Id;

use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Emoji {
    pub id: Id<Emoji>,
    pub name: String,
    pub url: Url,
    pub host: Option<String>,
    pub category: Option<String>,
    pub aliases: Vec<String>,
}

impl_entity!(Emoji);
