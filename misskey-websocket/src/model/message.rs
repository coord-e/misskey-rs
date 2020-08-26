use crate::model::RequestId;

use derive_more::Into;
use serde::de::{self, Deserializer};
use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum MessageType {
    Api(RequestId),
    Other(String),
}

impl<'de> Deserialize<'de> for MessageType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<MessageType, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MessageTypeVisitor;

        use std::{fmt, result};
        impl<'de> de::Visitor<'de> for MessageTypeVisitor {
            type Value = MessageType;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("message type")
            }

            fn visit_str<E>(self, value: &str) -> result::Result<Self::Value, E>
            where
                E: de::Error,
            {
                if let Some(id) = value.strip_prefix("api:") {
                    Ok(MessageType::Api(RequestId(id.to_string())))
                } else {
                    Ok(MessageType::Other(value.to_string()))
                }
            }
        }

        deserializer.deserialize_str(MessageTypeVisitor)
    }
}

#[derive(Deserialize, Debug, Clone, Into)]
pub(crate) struct ApiMessage {
    #[serde(default)]
    pub res: Value,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub(crate) enum OtherMessage {
    WithId {
        id: RequestId,
        #[serde(flatten)]
        content: Value,
    },
    WithoutId(Value),
}

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct Message {
    #[serde(rename = "type")]
    pub type_: MessageType,
    /// The deserialization of this value depends on `type_` and since we cannot express
    /// that constraint in the type, we keep this value as an untyped object here.
    pub body: Value,
}
