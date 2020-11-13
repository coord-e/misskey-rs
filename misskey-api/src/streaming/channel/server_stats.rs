use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MemStats {
    used: u64,
    active: u64,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NetStats {
    rx: f64,
    tx: f64,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FsStats {
    r: f64,
    w: f64,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ServerStats {
    pub cpu: f64,
    pub mem: MemStats,
    pub net: NetStats,
    pub fs: FsStats,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase", tag = "type", content = "body")]
pub enum ServerStatsEvent {
    Stats(ServerStats),
    StatsLog(Vec<ServerStats>),
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase", tag = "type", content = "body")]
pub enum Message {
    RequestLog { id: String, length: u64 },
}

#[derive(Serialize, Default, Debug, Clone)]
pub struct Request {}

impl misskey_core::streaming::ConnectChannelRequest for Request {
    type Incoming = ServerStatsEvent;
    type Outgoing = Message;

    const NAME: &'static str = "serverStats";
}

#[cfg(test)]
mod tests {
    use super::{Message, Request, ServerStatsEvent};
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
        tokio::time::timeout(Duration::from_millis(2100), async {
            loop {
                match stream.next().await.unwrap().unwrap() {
                    ServerStatsEvent::Stats(_) => break,
                    _ => continue,
                }
            }
        })
        .await
        .unwrap();
    }

    #[tokio::test]
    async fn stream_stats_log() {
        use ulid_crate::Ulid;

        let client = TestClient::new().await;
        let (mut sink, mut stream) = client.channel(Request::default()).await.unwrap().split();

        future::join(
            async {
                sink.send(Message::RequestLog {
                    id: Ulid::new().to_string(),
                    length: 50,
                })
                .await
                .unwrap();
            },
            async {
                loop {
                    match stream.next().await.unwrap().unwrap() {
                        ServerStatsEvent::StatsLog(_) => break,
                        _ => continue,
                    }
                }
            },
        )
        .await;
    }
}
