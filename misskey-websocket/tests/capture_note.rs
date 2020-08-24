mod common;
use common::{ClientExt, TestClient};

use futures::stream::StreamExt;
use misskey_api::api;
use misskey_api::model::note::Reaction;
use misskey_websocket::model::NoteUpdateEvent;

#[async_std::test]
async fn reacted() {
    let mut client = TestClient::new().await;
    let note = client
        .user
        .create_note(Some("looks good"), None, None)
        .await;

    let mut stream = client.user.capture_note(note.id.clone()).await.unwrap();

    futures::future::join(
        client.admin.test(api::notes::reactions::create::Request {
            note_id: note.id,
            reaction: Reaction("👍".to_string()),
        }),
        async {
            loop {
                match stream.next().await.unwrap().unwrap() {
                    NoteUpdateEvent::Reacted { .. } => break,
                    _ => continue,
                }
            }
        },
    )
    .await;
}

#[async_std::test]
async fn unreacted() {
    let mut client = TestClient::new().await;
    let note = client
        .user
        .create_note(Some("not so good"), None, None)
        .await;
    client
        .admin
        .test(api::notes::reactions::create::Request {
            note_id: note.id.clone(),
            reaction: Reaction("👍".to_string()),
        })
        .await;

    let mut stream = client.user.capture_note(note.id.clone()).await.unwrap();

    futures::future::join(
        client
            .admin
            .test(api::notes::reactions::delete::Request { note_id: note.id }),
        async {
            loop {
                match stream.next().await.unwrap().unwrap() {
                    NoteUpdateEvent::Unreacted { .. } => break,
                    _ => continue,
                }
            }
        },
    )
    .await;
}

#[async_std::test]
async fn deleted() {
    let mut client = TestClient::new().await;
    let note = client.user.create_note(Some("hmm..."), None, None).await;

    let mut stream = client.user.capture_note(note.id.clone()).await.unwrap();

    futures::future::join(
        client
            .user
            .test(api::notes::delete::Request { note_id: note.id }),
        async {
            loop {
                match stream.next().await.unwrap().unwrap() {
                    NoteUpdateEvent::Deleted { .. } => break,
                    _ => continue,
                }
            }
        },
    )
    .await;
}

#[async_std::test]
async fn poll_voted() {
    let mut client = TestClient::new().await;
    let poll = api::notes::create::PollRequest {
        choices: vec!["a".to_string(), "b".to_string()],
        multiple: Some(true),
        expires_at: Some(chrono::Utc::now() + chrono::Duration::hours(1)),
    };
    let note = client
        .user
        .test(api::notes::create::Request {
            visibility: None,
            visible_user_ids: Vec::new(),
            text: Some("?".to_string()),
            cw: None,
            via_mobile: false,
            local_only: false,
            no_extract_mentions: false,
            no_extract_hashtags: false,
            no_extract_emojis: false,
            file_ids: Vec::new(),
            reply_id: None,
            renote_id: None,
            poll: Some(poll),
        })
        .await
        .created_note;

    let mut stream = client.user.capture_note(note.id.clone()).await.unwrap();

    futures::future::join(
        client.user.test(api::notes::polls::vote::Request {
            note_id: note.id,
            choice: 0,
        }),
        async {
            loop {
                match stream.next().await.unwrap().unwrap() {
                    NoteUpdateEvent::PollVoted { .. } => break,
                    _ => continue,
                }
            }
        },
    )
    .await;
}
