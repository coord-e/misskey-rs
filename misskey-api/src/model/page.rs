use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(transparent)]
pub struct Page(pub serde_json::Value);
