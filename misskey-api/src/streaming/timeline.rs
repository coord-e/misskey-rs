use crate::model::{note::Note, timeline::Timeline};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Clone)]
pub struct Request {
    pub channel: Timeline,
}

#[derive(Deserialize, Debug, Clone)]
pub struct TimelineItem {
    pub body: Note,
}

impl misskey_core::streaming::SubscriptionRequest for Request {
    type Item = TimelineItem;
    const TYPE: &'static str = "connect";
}

impl misskey_core::streaming::SubscriptionItem for TimelineItem {
    const TYPE: &'static str = "channel";
    const UNSUBSCRIBE_REQUEST_TYPE: &'static str = "disconnect";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::model::timeline::Timeline;
    use crate::test::{websocket::TestClient, ClientExt};

    use futures::{future, StreamExt};
    use misskey_core::streaming::SubscriptionClient;

    async fn subscribe_unsubscribe_timeline(timeline: Timeline) {
        let mut client = TestClient::new().await;

        let mut stream = client
            .subscribe(Request { channel: timeline })
            .await
            .unwrap();
        stream.unsubscribe().await.unwrap();
    }

    #[tokio::test]
    async fn subscribe_unsubscribe_home() {
        subscribe_unsubscribe_timeline(Timeline::Home).await;
    }

    #[tokio::test]
    async fn subscribe_unsubscribe_local() {
        subscribe_unsubscribe_timeline(Timeline::Local).await;
    }

    #[tokio::test]
    async fn subscribe_unsubscribe_global() {
        subscribe_unsubscribe_timeline(Timeline::Global).await;
    }

    #[tokio::test]
    async fn subscribe_unsubscribe_socical() {
        subscribe_unsubscribe_timeline(Timeline::Social).await;
    }

    async fn stream_timeline(timeline: Timeline) {
        let mut client = TestClient::new().await;

        let mut stream = client
            .subscribe(Request { channel: timeline })
            .await
            .unwrap();

        future::join(
            client.create_note(Some("The world is fancy!"), None, None),
            async { stream.next().await.unwrap().unwrap() },
        )
        .await;
    }

    #[tokio::test]
    async fn stream_home() {
        stream_timeline(Timeline::Home).await;
    }

    #[tokio::test]
    async fn stream_local() {
        stream_timeline(Timeline::Local).await;
    }

    #[tokio::test]
    async fn stream_global() {
        stream_timeline(Timeline::Global).await;
    }

    #[tokio::test]
    async fn stream_social() {
        stream_timeline(Timeline::Social).await;
    }
}
