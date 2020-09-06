use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct QueueJobStats {
    pub active_since_prev_tick: u64,
    pub active: u64,
    pub waiting: u64,
    pub delayed: u64,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct QueueStats {
    pub deliver: QueueJobStats,
    pub inbox: QueueJobStats,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase", tag = "type", content = "body")]
pub enum QueueStatsEvent {
    Stats(QueueStats),
    StatsLog(Vec<QueueStats>),
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase", tag = "type", content = "body")]
pub enum Message {
    RequestLog { id: String, length: u64 },
}

#[derive(Serialize, Default, Debug, Clone)]
pub struct Request {}

impl misskey_core::streaming::ConnectChannelRequest for Request {
    type Incoming = QueueStatsEvent;
    type Outgoing = Message;

    const NAME: &'static str = "queueStats";
}

#[cfg(test)]
mod tests {
    use super::{Message, QueueStatsEvent, Request};
    use crate::test::websocket::TestClient;

    use futures::{future, SinkExt, StreamExt};

    #[tokio::test]
    async fn subscribe_unsubscribe() {
        let client = TestClient::new().await;
        let mut stream = client.channel(Request::default()).await.unwrap();
        stream.disconnect().await.unwrap();
    }

    #[tokio::test]
    async fn stream_stats() {
        use std::time::Duration;

        let client = TestClient::new().await;
        let mut stream = client.channel(Request::default()).await.unwrap();

        // margin of 100 ms
        tokio::time::timeout(Duration::from_millis(10100), async {
            loop {
                match stream.next().await.unwrap().unwrap() {
                    QueueStatsEvent::Stats(_) => break,
                    _ => continue,
                }
            }
        })
        .await
        .unwrap();
    }

    #[tokio::test]
    async fn stream_stats_log() {
        use uuid::Uuid;

        let client = TestClient::new().await;
        let (mut sink, mut stream) = client.channel(Request::default()).await.unwrap().split();

        future::join(
            async {
                sink.send(Message::RequestLog {
                    id: Uuid::new_v4().to_string(),
                    length: 50,
                })
                .await
                .unwrap();
            },
            async {
                loop {
                    match stream.next().await.unwrap().unwrap() {
                        QueueStatsEvent::StatsLog(_) => break,
                        _ => continue,
                    }
                }
            },
        )
        .await;
    }
}
