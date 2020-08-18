use crate::model::ChannelId;

use misskey::model::note::NoteId;
use serde::ser::Serializer;
use serde::Serialize;
use serde_json::value::Value;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase", tag = "type", content = "body")]
pub enum Request {
    Api {
        id: ChannelId,
        endpoint: String,
        data: Value,
    },
    Connect {
        id: ChannelId,
        channel: ConnectChannel,
    },
    SubNote {
        id: NoteId,
    },
    Disconnect {
        id: ChannelId,
    },
    UnsubNote {
        id: NoteId,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConnectChannel {
    Main,
    Timeline(Timeline),
}

impl Serialize for ConnectChannel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            ConnectChannel::Main => serializer.serialize_unit_variant("ConnectChannel", 0, "main"),
            ConnectChannel::Timeline(tl) => tl.serialize(serializer),
        }
    }
}

#[derive(Serialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Timeline {
    #[serde(rename = "homeTimeline")]
    Home,
    #[serde(rename = "localTimeline")]
    Local,
    #[serde(rename = "hybridTimeline")]
    Social,
    #[serde(rename = "globalTimeline")]
    Global,
}
