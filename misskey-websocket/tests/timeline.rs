mod common;
use common::{ClientExt, TestClient};

use futures::stream::StreamExt;
use misskey_websocket::model::Timeline;

async fn stream_timeline(timeline: Timeline) {
    let mut client = TestClient::new().await;

    let mut stream = client.user.timeline(timeline).await.unwrap();

    futures::future::join(
        client
            .user
            .create_note(Some("The world is fancy!"), None, None),
        async { stream.next().await.unwrap().unwrap() },
    )
    .await;
}

#[async_std::test]
async fn stream_home() {
    stream_timeline(Timeline::Home).await;
}

#[async_std::test]
async fn stream_local() {
    stream_timeline(Timeline::Local).await;
}

#[async_std::test]
async fn stream_global() {
    stream_timeline(Timeline::Global).await;
}

#[async_std::test]
async fn stream_social() {
    stream_timeline(Timeline::Social).await;
}
