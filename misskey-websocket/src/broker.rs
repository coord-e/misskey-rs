use std::sync::Arc;

use crate::channel::WebSocketReceriver;
use crate::error::Result;

use async_std::sync::RwLock;
use futures::stream::StreamExt;
use log::{debug, info};

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
                Ok(()) => {
                    info!("broker: exited normally");

                    let mut state = state.write().await;
                    *state = BrokerState::Exited;
                }
                Err(e) => {
                    let mut state = state.write().await;
                    *state = BrokerState::Dead(e);
                }
            }

            // This ensures that broker (and communication channels on broker side)
            // is dropped after `state` is surely set to `Dead` or `Exited`, thus asserts that the
            // state must be set to `Dead` or `Exited` when these channels are found out to be closed.
            std::mem::drop(broker);
        });

        (broker_tx, shared_state)
    }

    async fn clean_handler(&mut self) -> Result<()> {
        if self.handler.is_empty() {
            return Ok(());
        }

        debug!("broker: handler is not empty, enter receiving loop");
        while !self.handler.is_empty() {
            let msg = self.websocket_rx.recv_json().await?;
            debug!("broker: received {:?} (cleaning)", msg);

            self.handler.handle(msg).await?;
        }

        Ok(())
    }

    async fn task(&mut self) -> Result<()> {
        use futures::future::{self, Either};

        loop {
            let t1 = self.websocket_rx.recv_json();
            let t2 = self.broker_rx.next();

            futures::pin_mut!(t1, t2);

            match future::select(t1, t2).await {
                Either::Left((msg, _)) => {
                    let msg = msg?;
                    debug!("broker: received {:?}", msg);

                    while let Some(ctrl) = self.broker_rx.try_recv() {
                        debug!("broker: received control {:?}", ctrl);
                        self.handler.update(ctrl);
                    }

                    self.handler.handle(msg).await?;
                }
                Either::Right((Some(ctrl), _)) => {
                    debug!("broker: received control {:?}", ctrl);
                    self.handler.update(ctrl);
                }
                Either::Right((None, _)) => {
                    info!("broker: all controls terminated, exiting gracefully");
                    return self.clean_handler().await;
                }
            }
        }
    }
}
