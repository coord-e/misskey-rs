use std::fmt::{self, Debug};
use std::time::Duration;

use crate::channel::{connect_websocket, WebSocketReceiver};
use crate::error::{Error, Result};

#[cfg(all(not(feature = "tokio-runtime"), feature = "async-std-runtime"))]
use async_std::task;
#[cfg(all(not(feature = "tokio-runtime"), feature = "async-std-runtime"))]
use async_std::task::sleep as delay_for;
use async_tungstenite::tungstenite::Error as WsError;
use futures::sink::SinkExt;
use futures::stream::StreamExt;
use log::{debug, info, warn};
#[cfg(all(feature = "tokio-runtime", not(feature = "async-std-runtime")))]
use tokio::task;
#[cfg(all(feature = "tokio-runtime", not(feature = "async-std-runtime")))]
use tokio::time::delay_for;
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
    reconnect: Option<ReconnectConfig>,
    url: Url,
}

#[derive(Clone, Copy)]
pub enum ReconnectCondition {
    Always,
    UnexpectedReset,
    Custom(fn(&Error) -> bool),
}

impl Debug for ReconnectCondition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ReconnectCondition::Always => f.debug_tuple("Always").finish(),
            ReconnectCondition::UnexpectedReset => f.debug_tuple("UnexpectedReset").finish(),
            ReconnectCondition::Custom(_) => f.debug_tuple("Custom").finish(),
        }
    }
}

impl ReconnectCondition {
    fn should_reconnect(&self, err: &Error) -> bool {
        match self {
            ReconnectCondition::Always => true,
            ReconnectCondition::UnexpectedReset => {
                let ws = match err {
                    Error::WebSocket(ws) => ws,
                    _ => return false,
                };

                use std::io::ErrorKind;
                match ws.as_ref() {
                    WsError::Protocol(_) => true,
                    WsError::Io(e) => {
                        e.kind() == ErrorKind::ConnectionReset || e.kind() == ErrorKind::BrokenPipe
                    }
                    _ => false,
                }
            }
            ReconnectCondition::Custom(f) => f(err),
        }
    }
}

impl Default for ReconnectCondition {
    fn default() -> ReconnectCondition {
        ReconnectCondition::UnexpectedReset
    }
}

#[derive(Debug, Clone)]
pub struct ReconnectConfig {
    pub interval: Duration,
    pub condition: ReconnectCondition,
}

impl Default for ReconnectConfig {
    fn default() -> ReconnectConfig {
        ReconnectConfig {
            interval: Duration::from_secs(5),
            condition: ReconnectCondition::default(),
        }
    }
}

impl Broker {
    pub async fn spawn(
        url: Url,
        reconnect: Option<ReconnectConfig>,
    ) -> Result<(ControlSender, SharedBrokerState)> {
        let state = SharedBrokerState::working();
        let shared_state = SharedBrokerState::clone(&state);

        let (broker_tx, broker_rx) = control_channel(SharedBrokerState::clone(&state));

        task::spawn(async move {
            let mut broker = Broker {
                url,
                broker_rx,
                reconnect,
                handler: Handler::new(),
            };

            if let Some(err) = broker.run().await {
                state.set_error(err).await;
            } else {
                state.set_exited().await;
            }

            // This ensures that broker (and communication channels on broker side)
            // is dropped after `state` is surely set to `Dead` or `Exited`, thus asserts that the
            // state must be set to `Dead` or `Exited` when these channels are found out to be closed.
            std::mem::drop(broker);
        });

        Ok((broker_tx, shared_state))
    }

    async fn run(&mut self) -> Option<Error> {
        loop {
            let err = match self.task().await {
                Ok(()) => {
                    info!("broker: exited normally");
                    return None;
                }
                Err(e) => e,
            };

            info!("broker: task exited with error: {:?}", err);

            let config = match &self.reconnect {
                Some(config) if config.condition.should_reconnect(&err) => config,
                _ => {
                    warn!("broker: died with error");
                    return Some(err);
                }
            };

            info!("broker: attempt to reconnect in {:?}", config.interval);
            delay_for(config.interval).await;
        }
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

        for out in self.handler.restore_messages() {
            websocket_tx.send(out).await?;
        }

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
