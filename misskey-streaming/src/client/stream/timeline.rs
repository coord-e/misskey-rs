use std::pin::Pin;
use std::task::{Context, Poll};

use crate::broker::{
    channel::{ControlSender, ResponseStreamReceiver},
    model::BrokerControl,
};
use crate::model::ChannelId;

use futures::stream::Stream;
use misskey::model::note::Note;

pub struct Timeline {
    pub(in crate::client) id: ChannelId,
    pub(in crate::client) broker_tx: ControlSender,
    pub(in crate::client) response_rx: ResponseStreamReceiver<Note>,
}

impl Stream for Timeline {
    type Item = Note;
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Note>> {
        Pin::new(&mut self.response_rx).poll_next(cx)
    }
}

impl Drop for Timeline {
    fn drop(&mut self) {
        self.broker_tx
            .send(BrokerControl::UnsubscribeChannel(self.id.clone()))
    }
}
