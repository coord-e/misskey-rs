use crate::model::id::Id;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Page {
    pub id: Id<Page>,
    #[serde(flatten)]
    pub content: serde_json::Map<String, serde_json::Value>,
}

impl_entity!(Page);
