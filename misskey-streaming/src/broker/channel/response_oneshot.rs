use futures::channel::oneshot::{self, Receiver, Sender};

#[derive(Debug)]
pub struct ResponseSender<T>(Sender<T>);

impl<T> ResponseSender<T> {
    pub fn send(self, t: T) {
        if self.0.send(t).is_err() {
            panic!("oneshot broker response channel unexpectedly closed");
        }
    }
}

#[derive(Debug)]
pub struct ResponseReceiver<T>(Receiver<T>);

impl<T> ResponseReceiver<T> {
    pub async fn recv(self) -> T {
        self.0
            .await
            .expect("oneshot broker response channel unexpectedly closed")
    }
}

pub fn response_channel<T>() -> (ResponseSender<T>, ResponseReceiver<T>) {
    let (sender, receiver) = oneshot::channel();
    (ResponseSender(sender), ResponseReceiver(receiver))
}
