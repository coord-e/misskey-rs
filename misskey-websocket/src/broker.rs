use std::fmt::{self, Debug};
use std::sync::Arc;
use std::time::Duration;

use crate::channel::{connect_websocket, TrySendError, WebSocketReceiver};
use crate::error::{Error, Result};
use crate::model::outgoing::OutgoingMessage;

#[cfg(all(not(feature = "tokio-runtime"), feature = "async-std-runtime"))]
use async_std::task;
#[cfg(all(not(feature = "tokio-runtime"), feature = "async-std-runtime"))]
use async_std::task::sleep as delay_for;
use async_tungstenite::tungstenite::Error as WsError;
use futures::stream::StreamExt;
use log::{info, warn};
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

/// Specifies the condition for reconnecting.
#[derive(Clone)]
pub struct ReconnectCondition {
    inner: ReconnectConditionKind,
}

impl Debug for ReconnectCondition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("ReconnectCondition")
            .field(&self.inner)
            .finish()
    }
}

#[derive(Clone)]
enum ReconnectConditionKind {
    Always,
    Never,
    UnexpectedReset,
    // Using `Arc` instead of `Box` not to lose `Clone` for the infrequest use of `Custom` variant.
    Custom(Arc<dyn Fn(&Error) -> bool + Send + Sync + 'static>),
}

impl Debug for ReconnectConditionKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ReconnectConditionKind::Always => f.debug_tuple("Always").finish(),
            ReconnectConditionKind::Never => f.debug_tuple("Never").finish(),
            ReconnectConditionKind::UnexpectedReset => f.debug_tuple("UnexpectedReset").finish(),
            ReconnectConditionKind::Custom(_) => f.debug_tuple("Custom").finish(),
        }
    }
}

impl ReconnectCondition {
    /// Creates a `ReconnectCondition` that reconnects regardless of the errors.
    pub fn always() -> Self {
        ReconnectCondition {
            inner: ReconnectConditionKind::Always,
        }
    }

    /// Creates a `ReconnectCondition` that does not reconnect regardless of the errors.
    pub fn never() -> Self {
        ReconnectCondition {
            inner: ReconnectConditionKind::Never,
        }
    }

    /// Creates a `ReconnectCondition` that reconnects when the connection is lost unexpectedly.
    pub fn unexpected_reset() -> Self {
        ReconnectCondition {
            inner: ReconnectConditionKind::UnexpectedReset,
        }
    }

    /// Creates a custom `ReconnectCondition` using the passed function.
    pub fn custom<F>(f: F) -> Self
    where
        F: Fn(&Error) -> bool + Send + Sync + 'static,
    {
        ReconnectCondition {
            inner: ReconnectConditionKind::Custom(Arc::new(f)),
        }
    }

    fn should_reconnect(&self, err: &Error) -> bool {
        match &self.inner {
            ReconnectConditionKind::Always => true,
            ReconnectConditionKind::Never => false,
            ReconnectConditionKind::UnexpectedReset => {
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
            ReconnectConditionKind::Custom(f) => f(err),
        }
    }
}

impl Default for ReconnectCondition {
    /// [`unexpected_reset()`][`ReconnectCondition::unexpected_reset`] is used as a default.
    fn default() -> ReconnectCondition {
        ReconnectCondition::unexpected_reset()
    }
}

/// Reconnection configuration.
#[derive(Debug, Clone)]
pub struct ReconnectConfig {
    /// Sets an interval duration of automatic reconnection.
    pub interval: Duration,
    /// Specifies the condition for reconnecting.
    pub condition: ReconnectCondition,
    /// Specifies whether to re-send messages that may have failed to be sent when reconnecting.
    pub retry_send: bool,
}

impl ReconnectConfig {
    /// Creates a `ReconnectConfig` that disables reconnection.
    pub fn none() -> ReconnectConfig {
        ReconnectConfig::with_condition(ReconnectCondition::never())
    }

    /// Creates a `ReconnectConfig` with the given condition.
    pub fn with_condition(condition: ReconnectCondition) -> ReconnectConfig {
        ReconnectConfig {
            condition,
            ..Default::default()
        }
    }

    /// Creates a `ReconnectConfig` with the given interval.
    pub fn with_interval(interval: Duration) -> ReconnectConfig {
        ReconnectConfig {
            interval,
            ..Default::default()
        }
    }
}

impl Default for ReconnectConfig {
    /// `interval` is 5 secs and `retry_send` is `true` by default.
    fn default() -> ReconnectConfig {
        ReconnectConfig {
            interval: Duration::from_secs(5),
            condition: ReconnectCondition::default(),
            retry_send: true,
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
        let mut remaining_message = None;

        loop {
            let err = match self.task(remaining_message.take()).await {
                Ok(()) => {
                    info!("broker: exited normally");
                    return None;
                }
                Err(e) => e,
            };

            info!("broker: task exited with error: {:?}", err.error);

            let config = match &self.reconnect {
                Some(config) if config.condition.should_reconnect(&err.error) => config,
                _ => {
                    warn!("broker: died with error");
                    return Some(err.error);
                }
            };

            if config.retry_send {
                remaining_message = err.remaining_message;
            }

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

    async fn task(
        &mut self,
        remaining_message: Option<OutgoingMessage>,
    ) -> std::result::Result<(), TaskError> {
        use futures::future::{self, Either};

        let (mut websocket_tx, mut websocket_rx) = match connect_websocket(self.url.clone()).await {
            Ok(x) => x,
            Err(error) => {
                // retain `remaining_message` because we've not sent it yet
                return Err(TaskError {
                    remaining_message,
                    error,
                });
            }
        };

        info!("broker: started");

        if let Some(message) = remaining_message {
            websocket_tx.try_send(message).await?;
        }

        for message in self.handler.restore_messages() {
            websocket_tx.try_send(message).await?;
        }

        loop {
            let t1 = websocket_rx.recv();
            let t2 = self.broker_rx.next();

            futures::pin_mut!(t1, t2);

            match future::select(t1, t2).await {
                Either::Left((msg, _)) => {
                    while let Some(ctrl) = self.broker_rx.try_recv() {
                        #[cfg(feature = "inspect-contents")]
                        log::debug!("broker: received control {:?}", ctrl);

                        if let Some(out) = self.handler.control(ctrl) {
                            websocket_tx.try_send(out).await?
                        }
                    }

                    self.handler.handle(msg?).await?;
                }
                Either::Right((Some(ctrl), _)) => {
                    #[cfg(feature = "inspect-contents")]
                    log::debug!("broker: received control {:?}", ctrl);

                    if let Some(out) = self.handler.control(ctrl) {
                        websocket_tx.try_send(out).await?
                    }
                }
                Either::Right((None, _)) => {
                    info!("broker: all controls terminated, exiting gracefully");
                    return Ok(self.clean_handler(&mut websocket_rx).await?);
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
struct TaskError {
    remaining_message: Option<OutgoingMessage>,
    error: Error,
}

impl From<Error> for TaskError {
    fn from(error: Error) -> TaskError {
        TaskError {
            remaining_message: None,
            error,
        }
    }
}

impl From<TrySendError> for TaskError {
    fn from(err: TrySendError) -> TaskError {
        let TrySendError { message, error } = err;
        TaskError {
            remaining_message: Some(message),
            error,
        }
    }
}
