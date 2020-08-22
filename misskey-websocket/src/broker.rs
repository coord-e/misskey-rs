use std::sync::Arc;

use crate::channel::WebSocketReceriver;
use crate::error::Result;

use async_std::sync::RwLock;
use futures::never::Never;
use log::debug;

pub mod channel;
pub mod handler;
pub mod model;

use channel::{control_channel, ControlReceiver, ControlSender};
use handler::Handler;
use model::{BrokerState, SharedBrokerState};

#[derive(Debug)]
pub(crate) struct Broker {
    websocket_rx: WebSocketReceriver,
    broker_rx: ControlReceiver,
    handler: Handler,
}

impl Broker {
    pub fn spawn(websocket_rx: WebSocketReceriver) -> (ControlSender, SharedBrokerState) {
        let state = Arc::new(RwLock::new(BrokerState::Working));
        let shared_state = Arc::clone(&state);

        let (broker_tx, broker_rx) = control_channel(Arc::clone(&state));

        let mut broker = Broker {
            websocket_rx,
            broker_rx,
            handler: Handler::new(),
        };

        async_std::task::spawn(async move {
            match broker.task().await {
                Ok(x) => match x {},
                Err(e) => {
                    {
                        let mut state = state.write().await;
                        *state = BrokerState::Dead(e);
                    }

                    // This ensures that broker (and communication channels on broker side)
                    // is dropped after `state` is surely set to `Dead`, thus asserts that the
                    // state must be set to `Dead` when these channels are found out to be closed.
                    std::mem::drop(broker);
                }
            }
        });

        (broker_tx, shared_state)
    }

    async fn task(&mut self) -> Result<Never> {
        loop {
            let msg = self.websocket_rx.recv_json().await?;

            while let Some(ctrl) = self.broker_rx.try_recv() {
                debug!("received control {:?} (broker)", ctrl);
                self.handler.update(ctrl);
            }

            self.handler.handle(msg).await?;
        }
    }
}
