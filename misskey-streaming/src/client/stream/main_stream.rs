use std::pin::Pin;
use std::task::{Context, Poll};

use crate::broker::{
    channel::{ControlSender, ResponseStreamReceiver},
    model::BrokerControl,
};
use crate::model::{message::channel::MainStreamEvent, ChannelId};

use futures::stream::Stream;

pub struct MainStream {
    pub(in crate::client) id: ChannelId,
    pub(in crate::client) broker_tx: ControlSender,
    pub(in crate::client) response_rx: ResponseStreamReceiver<MainStreamEvent>,
}

impl Stream for MainStream {
    type Item = MainStreamEvent;
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<MainStreamEvent>> {
        Pin::new(&mut self.response_rx).poll_next(cx)
    }
}

impl Drop for MainStream {
    fn drop(&mut self) {
        self.broker_tx
            .send(BrokerControl::UnsubscribeChannel(self.id.clone()))
    }
}
