use crate::model::{note::Note, timeline::Timeline};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Clone)]
pub struct Request {
    pub channel: Timeline,
}

#[derive(Deserialize, Debug, Clone)]
pub struct TimelineItem {
    pub body: Note,
}

impl misskey_core::streaming::SubscriptionRequest for Request {
    type Item = TimelineItem;
    const TYPE: &'static str = "connect";
}

impl misskey_core::streaming::SubscriptionItem for TimelineItem {
    const TYPE: &'static str = "channel";
    const UNSUBSCRIBE_REQUEST_TYPE: &'static str = "disconnect";
}
