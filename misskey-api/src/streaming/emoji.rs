use crate::model::emoji::Emoji;

use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EmojiAddedEvent {
    pub emoji: Emoji,
}

impl misskey_core::streaming::BroadcastItem for EmojiAddedEvent {
    const TYPE: &'static str = "emojiAdded";
}
