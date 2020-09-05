use derive_more::{Display, FromStr, Into};
use misskey_core::streaming::SubNoteId;
use serde::de::{self, Deserializer};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash, FromStr, Debug, Display)]
#[serde(transparent)]
pub struct ChannelId(pub Uuid);

impl ChannelId {
    pub fn uuid() -> Self {
        ChannelId(Uuid::new_v4())
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, FromStr, Debug, Display)]
#[serde(transparent)]
pub struct ApiRequestId(pub Uuid);

impl ApiRequestId {
    pub fn uuid() -> Self {
        ApiRequestId(Uuid::new_v4())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IncomingMessageType {
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

                if let Some(id) = value.strip_prefix("api:") {
                    let uuid = Uuid::parse_str(id)
                        .map_err(|e| e.to_string())
                        .map_err(de::Error::custom)?;
                    Ok(IncomingMessageType::Api(ApiRequestId(uuid)))
                } else {
                    Ok(IncomingMessageType::Other(value.to_string()))
                }
            }
        }

        deserializer.deserialize_str(IncomingMessageTypeVisitor)
    }
}

#[derive(Deserialize, Debug, Clone, Into)]
pub struct ApiMessage {
    #[serde(default)]
    pub res: Value,
}

#[derive(Deserialize, Debug, Clone, Into)]
pub struct ChannelMessage {
    pub id: ChannelId,
    #[serde(flatten)]
    pub message: Value,
}

#[derive(Deserialize, Debug, Clone, Into)]
pub struct ConnectedMessage {
    pub id: ChannelId,
}

#[derive(Deserialize, Debug, Clone, Into)]
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

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase", tag = "type", content = "body")]
pub enum OutgoingMessage {
    Api {
        id: ApiRequestId,
        endpoint: &'static str,
        data: Value,
    },
    Connect {
        id: ChannelId,
        channel: &'static str,
        params: Value,
        pong: bool,
    },
    Channel {
        id: ChannelId,
        #[serde(flatten)]
        message: Value,
    },
    Disconnect {
        id: ChannelId,
    },
    SubNote {
        id: SubNoteId,
    },
    UnsubNote {
        id: SubNoteId,
    },
}
