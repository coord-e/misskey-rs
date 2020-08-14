use crate::channel::WebSocketReceriver;
use crate::error::Result;

use futures::never::Never;
use log::debug;

pub mod channel;
pub mod handler;
pub mod model;

use channel::control::ControlReceiver;
use handler::Handler;

pub struct Broker {
    websocket_rx: WebSocketReceriver,
    broker_rx: ControlReceiver,
}

impl Broker {
    pub fn new(websocket_rx: WebSocketReceriver, broker_rx: ControlReceiver) -> Broker {
        Broker {
            websocket_rx,
            broker_rx,
        }
    }

    async fn broker(mut self) -> Result<Never> {
        let mut handler = Handler::new();

        loop {
            let msg = self.websocket_rx.recv_json().await?;

            while let Some(ctrl) = self.broker_rx.try_recv() {
                debug!("received control {:?} (broker)", ctrl);
                handler.update(ctrl);
            }

            handler.handle(msg).await?;
        }
    }

    pub fn spawn(self) {
        tokio::spawn(async {
            loop {
                match self.broker().await {
                    Ok(x) => match x {},
                    // TODO: handle
                    Err(e) => panic!("ws: {}", e),
                }
            }
        });
    }
}
