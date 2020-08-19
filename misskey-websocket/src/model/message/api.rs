use derive_more::Into;
use serde::Deserialize;
use serde_json::value::Value;

#[derive(Deserialize, Debug, Clone, Into)]
pub(crate) struct ApiMessage {
    #[serde(default = "default_null")]
    pub res: Value,
}

fn default_null() -> Value {
    Value::Null
}
