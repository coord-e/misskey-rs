use crate::model::{id::Id, note::Reaction, user::User};

use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase", tag = "type", content = "body")]
pub enum NoteUpdateEvent {
    #[serde(rename_all = "camelCase")]
    Reacted {
        reaction: Reaction,
        user_id: Id<User>,
    },
    #[serde(rename_all = "camelCase")]
    Unreacted {
        reaction: Reaction,
        user_id: Id<User>,
    },
    #[serde(rename_all = "camelCase")]
    Deleted { deleted_at: DateTime<Utc> },
    #[serde(rename_all = "camelCase")]
    PollVoted { choice: u64, user_id: Id<User> },
}

impl misskey_core::streaming::SubNoteEvent for NoteUpdateEvent {}

#[cfg(test)]
mod tests {
    use super::NoteUpdateEvent;
    use crate::test::{websocket::TestClient, ClientExt};

    use futures::{future, StreamExt};

    #[tokio::test]
    async fn subscribe_unsubscribe() {
        let client = TestClient::new().await;
        let note = client.create_note(Some("test"), None, None).await;

        let mut stream = client
            .subnote::<NoteUpdateEvent, _>(note.id.to_string())
            .await
            .unwrap();
        stream.unsubscribe().await.unwrap();
    }

    #[tokio::test]
    async fn reacted() {
        use crate::model::note::Reaction;

        let client = TestClient::new().await;
        let note = client
            .user
            .create_note(Some("looks good"), None, None)
            .await;

        let mut stream = client.user.subnote(note.id.to_string()).await.unwrap();

        future::join(
            client
                .admin
                .test(crate::endpoint::notes::reactions::create::Request {
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

    #[tokio::test]
    async fn unreacted() {
        use crate::model::note::Reaction;

        let client = TestClient::new().await;
        let note = client
            .user
            .create_note(Some("not so good"), None, None)
            .await;
        client
            .admin
            .test(crate::endpoint::notes::reactions::create::Request {
                note_id: note.id.clone(),
                reaction: Reaction("👍".to_string()),
            })
            .await;

        let mut stream = client.user.subnote(note.id.to_string()).await.unwrap();

        future::join(
            client
                .admin
                .test(crate::endpoint::notes::reactions::delete::Request { note_id: note.id }),
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

    #[tokio::test]
    async fn deleted() {
        let client = TestClient::new().await;
        let note = client.user.create_note(Some("hmm..."), None, None).await;

        let mut stream = client.user.subnote(note.id.to_string()).await.unwrap();

        future::join(
            client
                .user
                .test(crate::endpoint::notes::delete::Request { note_id: note.id }),
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

    #[tokio::test]
    async fn poll_voted() {
        let client = TestClient::new().await;
        let poll = crate::endpoint::notes::create::PollRequest {
            choices: vec!["a".to_string(), "b".to_string()],
            multiple: Some(true),
            expires_at: Some(chrono::Utc::now() + chrono::Duration::hours(1)),
            expired_after: None,
        };
        let note = client
            .user
            .test(crate::endpoint::notes::create::Request {
                visibility: None,
                visible_user_ids: None,
                text: Some("?".to_string()),
                cw: None,
                via_mobile: None,
                local_only: None,
                no_extract_mentions: None,
                no_extract_hashtags: None,
                no_extract_emojis: None,
                file_ids: None,
                reply_id: None,
                renote_id: None,
                poll: Some(poll),
                #[cfg(feature = "12-47-0")]
                channel_id: None,
            })
            .await
            .created_note;

        let mut stream = client.user.subnote(note.id.to_string()).await.unwrap();

        futures::future::join(
            client
                .user
                .test(crate::endpoint::notes::polls::vote::Request {
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
}
