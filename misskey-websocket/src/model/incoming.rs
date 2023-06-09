#[cfg(not(feature = "12-111-0"))]
use crate::model::ApiRequestId;
use crate::model::{ChannelId, SubNoteId};

use serde::de::{self, Deserializer};
use serde::Deserialize;
use serde_json::Value;
#[cfg(not(feature = "12-111-0"))]
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IncomingMessageType {
    #[cfg(not(feature = "12-111-0"))]
    Api(ApiRequestId),
    Channel,
    Connected,
    NoteUpdated,
    Other(String),
}

impl<'de> Deserialize<'de> for IncomingMessageType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<IncomingMessageType, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IncomingMessageTypeVisitor;

        use std::{fmt, result};
        impl<'de> de::Visitor<'de> for IncomingMessageTypeVisitor {
            type Value = IncomingMessageType;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("message type")
            }

            fn visit_str<E>(self, value: &str) -> result::Result<Self::Value, E>
            where
                E: de::Error,
            {
                match value {
                    "channel" => return Ok(IncomingMessageType::Channel),
                    "connected" => return Ok(IncomingMessageType::Connected),
                    "noteUpdated" => return Ok(IncomingMessageType::NoteUpdated),
                    _ => (),
                }

                #[cfg(not(feature = "12-111-0"))]
                if let Some(id) = value.strip_prefix("api:") {
                    let uuid = Uuid::try_parse(id)
                        .map_err(|e| e.to_string())
                        .map_err(de::Error::custom)?;
                    Ok(IncomingMessageType::Api(ApiRequestId(uuid)))
                } else {
                    Ok(IncomingMessageType::Other(value.to_string()))
                }

                #[cfg(feature = "12-111-0")]
                Ok(IncomingMessageType::Other(value.to_string()))
            }
        }

        deserializer.deserialize_str(IncomingMessageTypeVisitor)
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct ApiMessage {
    #[serde(default)]
    pub res: Value,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ChannelMessage {
    pub id: ChannelId,
    #[serde(flatten)]
    pub message: Value,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ConnectedMessage {
    pub id: ChannelId,
}

#[derive(Deserialize, Debug, Clone)]
pub struct NoteUpdatedMessage {
    pub id: SubNoteId,
    #[serde(flatten)]
    pub message: Value,
}

#[derive(Deserialize, Debug, Clone)]
pub struct IncomingMessage {
    #[serde(rename = "type")]
    pub type_: IncomingMessageType,
    pub body: Value,
}
