mod common;
use common::{ClientExt, TestClient};

use futures::stream::StreamExt;
use misskey_websocket::model::MainStreamEvent;

#[async_std::test]
async fn reply() {
    let mut client = TestClient::new().await;

    let mut stream = client.user.main_stream().await.unwrap();

    futures::future::join(
        async {
            let note = client.user.create_note(Some("awesome"), None, None).await;
            client
                .admin
                .create_note(Some("nice"), None, Some(note.id))
                .await;
        },
        async {
            loop {
                match stream.next().await.unwrap().unwrap() {
                    MainStreamEvent::Reply(_) => break,
                    _ => continue,
                }
            }
        },
    )
    .await;
}

#[async_std::test]
async fn mention() {
    let mut client = TestClient::new().await;
    let me = client.user.me().await;

    let mut stream = client.user.main_stream().await.unwrap();

    futures::future::join(
        client
            .admin
            .create_note(Some(&format!("@{} hello", me.username)), None, None),
        async {
            loop {
                match stream.next().await.unwrap().unwrap() {
                    MainStreamEvent::Mention(_) => break,
                    _ => continue,
                }
            }
        },
    )
    .await;
}

// TODO: Test the other events
