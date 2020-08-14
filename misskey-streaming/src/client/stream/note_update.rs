use std::pin::Pin;
use std::task::{Context, Poll};

use crate::broker::{
    channel::{ControlSender, ResponseStreamReceiver},
    model::BrokerControl,
};
use crate::model::message::note_updated::NoteUpdateEvent;

use futures::stream::Stream;
use misskey::model::note::NoteId;

pub struct NoteUpdate {
    pub(in crate::client) id: NoteId,
    pub(in crate::client) broker_tx: ControlSender,
    pub(in crate::client) response_rx: ResponseStreamReceiver<NoteUpdateEvent>,
}

impl Stream for NoteUpdate {
    type Item = NoteUpdateEvent;
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<NoteUpdateEvent>> {
        Pin::new(&mut self.response_rx).poll_next(cx)
    }
}

impl Drop for NoteUpdate {
    fn drop(&mut self) {
        self.broker_tx
            .send(BrokerControl::UnsubscribeNote(self.id.clone()));
    }
}
