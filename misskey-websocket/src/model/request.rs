use crate::model::ChannelId;

use misskey::model::note::NoteId;
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

#[derive(Serialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum ConnectChannel {
    Main(MainType),
    Timeline(TimelineType),
}

#[derive(Serialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MainType {
    #[serde(rename = "main")]
    Main,
}

#[derive(Serialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TimelineType {
    #[serde(rename = "homeTimeline")]
    Home,
    #[serde(rename = "localTimeline")]
    Local,
    #[serde(rename = "hybridTimeline")]
    Social,
    #[serde(rename = "globalTimeline")]
    Global,
}
