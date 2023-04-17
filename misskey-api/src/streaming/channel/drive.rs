use crate::model::{
    drive::{DriveFile, DriveFolder},
    id::Id,
};
use crate::streaming::channel::NoOutgoing;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase", tag = "type", content = "body")]
pub enum DriveStreamEvent {
    FolderCreated(DriveFolder),
    FolderUpdated(DriveFolder),
    FolderDeleted(Id<DriveFolder>),
    FileCreated(DriveFile),
    FileUpdated(DriveFile),
    FileDeleted(Id<DriveFile>),
}

#[derive(Serialize, Default, Debug, Clone)]
pub struct Request {}

impl misskey_core::streaming::ConnectChannelRequest for Request {
    type Incoming = DriveStreamEvent;
    type Outgoing = NoOutgoing;

    const NAME: &'static str = "drive";
}

#[cfg(test)]
mod tests {
    use super::{DriveStreamEvent, Request};
    use crate::test::{websocket::TestClient, ClientExt};

    use futures::{future, StreamExt};

    #[tokio::test]
    async fn subscribe_unsubscribe() {
        let client = TestClient::new().await;
        let mut stream = client.channel(Request::default()).await.unwrap();
        stream.disconnect().await.unwrap();
    }

    #[tokio::test]
    async fn stream_folder_created() {
        let client = TestClient::new().await;
        let mut stream = client.channel(Request::default()).await.unwrap();

        future::join(
            client.test(crate::endpoint::drive::folders::create::Request::default()),
            async {
                loop {
                    match stream.next().await.unwrap().unwrap() {
                        DriveStreamEvent::FolderCreated(_) => break,
                        _ => continue,
                    }
                }
            },
        )
        .await;
    }

    #[tokio::test]
    async fn stream_folder_updated() {
        let client = TestClient::new().await;
        let folder = client
            .test(crate::endpoint::drive::folders::create::Request::default())
            .await;
        let mut stream = client.channel(Request::default()).await.unwrap();

        future::join(
            client.test(crate::endpoint::drive::folders::update::Request {
                folder_id: folder.id,
                name: Some("test".to_string()),
                parent_id: None,
            }),
            async {
                loop {
                    match stream.next().await.unwrap().unwrap() {
                        DriveStreamEvent::FolderUpdated(_) => break,
                        _ => continue,
                    }
                }
            },
        )
        .await;
    }

    #[tokio::test]
    async fn stream_folder_deleted() {
        let client = TestClient::new().await;
        let folder = client
            .test(crate::endpoint::drive::folders::create::Request::default())
            .await;
        let mut stream = client.channel(Request::default()).await.unwrap();

        future::join(
            client.test(crate::endpoint::drive::folders::delete::Request {
                folder_id: folder.id,
            }),
            async {
                loop {
                    match stream.next().await.unwrap().unwrap() {
                        DriveStreamEvent::FolderDeleted(_) => break,
                        _ => continue,
                    }
                }
            },
        )
        .await;
    }

    #[tokio::test]
    async fn stream_file_created() {
        let client = TestClient::new().await;
        let url = client.avatar_url().await;
        let mut stream = client.channel(Request::default()).await.unwrap();

        future::join(client.upload_from_url(url), async {
            loop {
                match stream.next().await.unwrap().unwrap() {
                    DriveStreamEvent::FileCreated(_) => break,
                    _ => continue,
                }
            }
        })
        .await;
    }

    #[tokio::test]
    async fn stream_file_updated() {
        let client = TestClient::new().await;
        let url = client.avatar_url().await;
        let file = client.upload_from_url(url).await;
        let mut stream = client.channel(Request::default()).await.unwrap();

        future::join(
            client.test(
                crate::endpoint::drive::files::update::Request::builder()
                    .file_id(file.id)
                    .name("test")
                    .build(),
            ),
            async {
                loop {
                    match stream.next().await.unwrap().unwrap() {
                        DriveStreamEvent::FileUpdated(_) => break,
                        _ => continue,
                    }
                }
            },
        )
        .await;
    }

    #[tokio::test]
    async fn stream_file_deleted() {
        let client = TestClient::new().await;
        let url = client.avatar_url().await;
        let file = client.upload_from_url(url).await;
        let mut stream = client.channel(Request::default()).await.unwrap();

        future::join(
            client.test(crate::endpoint::drive::files::delete::Request { file_id: file.id }),
            async {
                loop {
                    match stream.next().await.unwrap().unwrap() {
                        DriveStreamEvent::FileDeleted(_) => break,
                        _ => continue,
                    }
                }
            },
        )
        .await;
    }
}
