use crate::channel::{connect_websocket, WebSocketReceiver};
use crate::error::Result;

#[cfg(all(not(feature = "tokio-runtime"), feature = "async-std-runtime"))]
use async_std::task;
use futures::sink::SinkExt;
use futures::stream::StreamExt;
use log::{debug, info, warn};
#[cfg(all(feature = "tokio-runtime", not(feature = "async-std-runtime")))]
use tokio::task;
use url::Url;

pub mod channel;
pub mod handler;
pub mod model;

use channel::{control_channel, ControlReceiver, ControlSender};
use handler::Handler;
use model::SharedBrokerState;

#[derive(Debug)]
pub(crate) struct Broker {
    broker_rx: ControlReceiver,
    handler: Handler,
    url: Url,
}

impl Broker {
    pub async fn spawn(url: Url) -> Result<(ControlSender, SharedBrokerState)> {
        let state = SharedBrokerState::working();
        let shared_state = SharedBrokerState::clone(&state);

        let (broker_tx, broker_rx) = control_channel(SharedBrokerState::clone(&state));

        let mut broker = Broker {
            url,
            broker_rx,
            handler: Handler::new(),
        };

        task::spawn(async move {
            match broker.task().await {
                Ok(()) => {
                    info!("broker: exited normally");
                    state.set_exited().await;
                }
                Err(e) => {
                    warn!("broker: exited with error: {:?}", e);
                    state.set_error(e).await;
                }
            }

            // This ensures that broker (and communication channels on broker side)
            // is dropped after `state` is surely set to `Dead` or `Exited`, thus asserts that the
            // state must be set to `Dead` or `Exited` when these channels are found out to be closed.
            std::mem::drop(broker);
        });

        Ok((broker_tx, shared_state))
    }

    async fn clean_handler(&mut self, websocket_rx: &mut WebSocketReceiver) -> Result<()> {
        if self.handler.is_empty() {
            return Ok(());
        }

        info!("broker: handler is not empty, enter receiving loop");
        while !self.handler.is_empty() {
            let msg = websocket_rx.recv().await?;
            self.handler.handle(msg).await?;
        }

        Ok(())
    }

    async fn task(&mut self) -> Result<()> {
        use futures::future::{self, Either};

        let (mut websocket_tx, mut websocket_rx) = connect_websocket(self.url.clone()).await?;

        info!("broker: started");

        loop {
            let t1 = websocket_rx.recv();
            let t2 = self.broker_rx.next();

            futures::pin_mut!(t1, t2);

            match future::select(t1, t2).await {
                Either::Left((msg, _)) => {
                    while let Some(ctrl) = self.broker_rx.try_recv() {
                        #[cfg(feature = "inspect-contents")]
                        debug!("broker: received control {:?}", ctrl);

                        if let Some(out) = self.handler.control(ctrl) {
                            websocket_tx.send(out).await?;
                        }
                    }

                    self.handler.handle(msg?).await?;
                }
                Either::Right((Some(ctrl), _)) => {
                    #[cfg(feature = "inspect-contents")]
                    debug!("broker: received control {:?}", ctrl);

                    if let Some(out) = self.handler.control(ctrl) {
                        websocket_tx.send(out).await?;
                    }
                }
                Either::Right((None, _)) => {
                    info!("broker: all controls terminated, exiting gracefully");
                    return self.clean_handler(&mut websocket_rx).await;
                }
            }
        }
    }
}
