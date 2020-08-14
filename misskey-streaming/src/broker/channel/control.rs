use std::pin::Pin;
use std::task::{Context, Poll};

use crate::broker::model::BrokerControl;

use futures::channel::mpsc::{self, UnboundedReceiver, UnboundedSender};
use futures::stream::Stream;

/// Sender channel that the client uses to communicate with broker
#[derive(Debug, Clone)]
pub struct ControlSender(UnboundedSender<BrokerControl>);

impl ControlSender {
    pub fn send(&mut self, ctrl: BrokerControl) {
        self.0
            .unbounded_send(ctrl)
            .expect("broker channel unexpectedly closed")
    }
}

/// Receiver channel that broker uses to communicate with the client
#[derive(Debug)]
pub struct ControlReceiver(UnboundedReceiver<BrokerControl>);

impl ControlReceiver {
    pub fn try_recv(&mut self) -> Option<BrokerControl> {
        match self.0.try_next() {
            Ok(Some(x)) => Some(x),
            Ok(None) => panic!("try_next returned None but we never closed the channel"),
            Err(_) => None,
        }
    }
}

impl Stream for ControlReceiver {
    type Item = BrokerControl;
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<BrokerControl>> {
        Pin::new(&mut self.0).poll_next(cx)
    }
}

pub fn control_channel() -> (ControlSender, ControlReceiver) {
    let (sender, receiver) = mpsc::unbounded();
    (ControlSender(sender), ControlReceiver(receiver))
}
