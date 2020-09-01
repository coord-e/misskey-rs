use crate::model::request::ApiRequestId;

use derive_more::{Display, FromStr, Into};
use serde::de::{self, Deserializer};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, FromStr, Debug, Display)]
pub(crate) struct SubscriptionId(pub String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum MessageType {
    Api(ApiRequestId),
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
                    let uuid = Uuid::parse_str(id)
                        .map_err(|e| e.to_string())
                        .map_err(de::Error::custom)?;
                    Ok(MessageType::Api(ApiRequestId(uuid)))
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
    Subscription {
        id: SubscriptionId,
        #[serde(flatten)]
        content: Value,
    },
    Broadcast(Value),
}

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct Message {
    #[serde(rename = "type")]
    pub type_: MessageType,
    /// The deserialization of this value depends on `type_` and since we cannot express
    /// that constraint in the type, we keep this value as an untyped object here.
    pub body: Value,
}
