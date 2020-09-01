use crate::model::note::Note;

use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct HybridTimeline {
    #[serde(rename = "body")]
    pub note: Note,
}

impl crate::streaming::channel::Channel for HybridTimeline {
    const NAME: &'static str = "hybridTimeline";
}

#[cfg(test)]
mod tests {
    use super::HybridTimeline;
    use crate::streaming::channel::ConnectRequest;
    use crate::test::{websocket::TestClient, ClientExt};

    use futures::{future, StreamExt};
    use misskey_core::streaming::SubscriptionClient;

    #[tokio::test]
    async fn subscribe_unsubscribe() {
        let mut client = TestClient::new().await;

        let mut stream = client
            .subscribe(ConnectRequest::<HybridTimeline>::new())
            .await
            .unwrap();
        stream.unsubscribe().await.unwrap();
    }

    #[tokio::test]
    async fn stream() {
        let mut client = TestClient::new().await;

        let mut stream = client
            .subscribe(ConnectRequest::<HybridTimeline>::new())
            .await
            .unwrap();

        future::join(
            client.create_note(Some("The world is fancy!"), None, None),
            async { stream.next().await.unwrap().unwrap() },
        )
        .await;
    }
}
