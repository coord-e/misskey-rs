use std::hash::Hash;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

#[cfg(not(feature = "12-111-0"))]
use crate::broker::channel::ResponseSender;
use crate::broker::channel::{ChannelPongSender, ResponseStreamSender};
use crate::error::Error;
#[cfg(not(feature = "12-111-0"))]
use crate::model::ApiRequestId;
use crate::model::{ChannelId, SubNoteId};

use async_rwlock::RwLock;
use futures_util::future::{BoxFuture, Future, FutureExt};
#[cfg(not(feature = "12-111-0"))]
use misskey_core::model::ApiResult;
use serde_json::Value;
use uuid::Uuid;

#[derive(Clone, PartialEq, Eq, Hash, Debug, Copy)]
pub(crate) struct BroadcastId(pub Uuid);

impl BroadcastId {
    pub fn new() -> Self {
        BroadcastId(Uuid::new_v4())
    }
}

#[derive(Debug)]
pub(crate) enum BrokerControl {
    #[cfg(not(feature = "12-111-0"))]
    Api {
        id: ApiRequestId,
        endpoint: &'static str,
        data: Value,
        sender: ResponseSender<ApiResult<Value>>,
    },
    Connect {
        id: ChannelId,
        name: &'static str,
        params: Value,
        sender: ResponseStreamSender<Value>,
        pong: ChannelPongSender,
    },
    Channel {
        id: ChannelId,
        message: Value,
    },
    Disconnect {
        id: ChannelId,
    },
    SubNote {
        id: SubNoteId,
        sender: ResponseStreamSender<Value>,
    },
    UnsubNote {
        id: SubNoteId,
    },
    StartBroadcast {
        id: BroadcastId,
        type_: &'static str,
        sender: ResponseStreamSender<Value>,
    },
    StopBroadcast {
        id: BroadcastId,
    },
}

#[derive(Debug, Clone)]
pub(crate) enum BrokerState {
    /// Broker is properly working and is available.
    Working,
    /// Broker is exited properly. (unavailable)
    Exited,
    /// Broker is exited with an error. (unavailable)
    Dead(Error),
}

impl BrokerState {
    pub fn dead(self) -> Option<Error> {
        match self {
            BrokerState::Working => None,
            // TODO: clearify the guarantee that no one asks for `BrokerState` after broker is dead
            BrokerState::Exited => panic!("asked if broker is dead while it is already exited"),
            BrokerState::Dead(e) => Some(e),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct SharedBrokerState(Arc<RwLock<BrokerState>>);

pub(crate) struct ReadBrokerState(BoxFuture<'static, BrokerState>);

impl Future for ReadBrokerState {
    type Output = BrokerState;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<BrokerState> {
        self.0.poll_unpin(cx)
    }
}

impl SharedBrokerState {
    pub fn working() -> SharedBrokerState {
        SharedBrokerState(Arc::new(RwLock::new(BrokerState::Working)))
    }

    pub async fn set_exited(&self) {
        let mut lock = self.0.write().await;
        *lock = BrokerState::Exited;
    }

    pub async fn set_error(&self, err: Error) {
        let mut lock = self.0.write().await;
        *lock = BrokerState::Dead(err);
    }

    /// `None` means that broker is during its exit by some reason. (thus unavailable)
    pub fn try_read(&self) -> Option<BrokerState> {
        self.0.try_read().map(|lock| BrokerState::clone(&*lock))
    }

    pub fn read(&self) -> ReadBrokerState {
        let p = Arc::clone(&self.0);
        ReadBrokerState(Box::pin(async move {
            let lock = p.read().await;
            BrokerState::clone(&*lock)
        }))
    }
}
