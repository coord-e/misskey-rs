use crate::model::ChannelId;

use serde::de::{self, Deserializer};
use serde::Deserialize;
use serde_json::value::Value;
use uuid::Uuid;

pub mod api;
pub mod channel;
pub mod note_updated;

#[derive(Debug)]
pub enum MessageType {
    Api(ChannelId),
    Channel,
    NoteUpdated,
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
                match value {
                    "channel" => return Ok(MessageType::Channel),
                    "noteUpdated" => return Ok(MessageType::NoteUpdated),
                    _ => (),
                }

                if let Some(id) = value.strip_prefix("api:") {
                    let uuid = Uuid::parse_str(id)
                        .map_err(|e| e.to_string())
                        .map_err(de::Error::custom)?;
                    Ok(MessageType::Api(ChannelId(uuid)))
                } else {
                    Err(de::Error::unknown_variant(
                        value,
                        &["api:<id>", "channel", "noteUpdated"],
                    ))
                }
            }
        }

        deserializer.deserialize_str(MessageTypeVisitor)
    }
}

#[derive(Deserialize, Debug)]
pub struct Message {
    #[serde(rename = "type")]
    pub type_: MessageType,
    /// This value would be deserialized into one of `ApiMessage`, `ChannelMessage`, `NotePostedMessage`,
    /// or `NoteUpdatedMessage`. But this choice depends on `type_` and since we cannot express
    /// that constraint in the type, we keep this value as `Value` here.
    pub body: Value,
}
