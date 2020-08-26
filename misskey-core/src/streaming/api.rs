use serde::de::DeserializeOwned;
use serde::Serialize;

pub trait SubscriptionRequest: Serialize {
    type Item: SubscriptionItem;
    const TYPE: &'static str;
}

pub trait SubscriptionItem: DeserializeOwned {
    const TYPE: &'static str;
    const UNSUBSCRIBE_REQUEST_TYPE: &'static str;
}

pub trait BroadcastItem: DeserializeOwned {
    const TYPE: &'static str;
}
