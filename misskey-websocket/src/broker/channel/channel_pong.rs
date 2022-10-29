use crate::broker::model::SharedBrokerState;
use crate::error::Result;

use futures_channel::oneshot::{self, Receiver, Sender};

#[derive(Debug)]
pub(crate) struct ChannelPongSender(Sender<()>);

impl ChannelPongSender {
    pub fn send(self) {
        if self.0.send(()).is_err() {
            panic!("oneshot channel pong channel unexpectedly closed");
        }
    }
}

#[derive(Debug)]
pub(crate) struct ChannelPongReceiver {
    inner: Receiver<()>,
    state: SharedBrokerState,
}

impl ChannelPongReceiver {
    pub async fn recv(self) -> Result<()> {
        match self.inner.await {
            Ok(()) => Ok(()),
            Err(_) => {
                let state = self.state.read().await;
                let err = state
                    .dead()
                    .expect("channel pong channel unexpectedly closed");
                Err(err)
            }
        }
    }
}

pub(crate) fn channel_pong_channel(
    state: SharedBrokerState,
) -> (ChannelPongSender, ChannelPongReceiver) {
    let (sender, receiver) = oneshot::channel();
    (
        ChannelPongSender(sender),
        ChannelPongReceiver {
            inner: receiver,
            state,
        },
    )
}
