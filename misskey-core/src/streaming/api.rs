use derive_more::{Display, FromStr};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

pub trait Request: Serialize {
    const TYPE: &'static str;
}

impl<T: ?Sized> Request for &'_ T
where
    T: Request,
{
    const TYPE: &'static str = T::TYPE;
}

pub trait OneshotRequest: Request {}

pub trait BroadcastMessage: DeserializeOwned {
    const TYPE: &'static str;
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, FromStr, Debug, Display)]
#[serde(transparent)]
pub struct SubscriptionId(pub String);

pub trait SubscribeRequest: Request {
    type Content: SubscriptionContent;
    type Unsubscribe: UnsubscribeRequest<Id = Self::Id>;

    type Id: Into<SubscriptionId> + Clone;
    fn id(&self) -> &Self::Id;
}

pub trait SubscriptionContent: DeserializeOwned {
    const MESSAGE_TYPE: &'static str;
}

pub trait UnsubscribeRequest: Request {
    type Id;
    fn from_id(id: Self::Id) -> Self;
}
