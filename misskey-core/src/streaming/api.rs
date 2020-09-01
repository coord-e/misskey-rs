use serde::de::DeserializeOwned;
use serde::Serialize;

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

pub trait UnsubscribeRequest: Request {
    type Id;
    fn from_id(id: Self::Id) -> Self;
}

pub trait SubscribeRequest: Request {
    type Message: SubscriptionMessage;
    type Unsubscribe: UnsubscribeRequest<Id = Self::Id>;

    type Id: Serialize + Clone;
    fn id(&self) -> &Self::Id;
}

pub trait Message: DeserializeOwned {
    const TYPE: &'static str;
}

pub trait BroadcastMessage: Message {}

pub trait SubscriptionMessage: Message {}
