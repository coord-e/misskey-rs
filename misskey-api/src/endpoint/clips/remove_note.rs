use crate::model::{clip::Clip, id::Id, note::Note};

use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub clip_id: Id<Clip>,
    pub note_id: Id<Note>,
}

impl misskey_core::Request for Request {
    type Response = ();
    const ENDPOINT: &'static str = "clips/remove-note";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let client = TestClient::new();
        let note = client.create_note(Some("test"), None, None).await;
        let clip = client
            .test(
                crate::endpoint::clips::create::Request::builder()
                    .name("testclip".to_string())
                    .build(),
            )
            .await;
        client
            .user
            .test(crate::endpoint::clips::add_note::Request {
                clip_id: clip.id,
                note_id: note.id,
            })
            .await;

        client
            .test(Request {
                clip_id: clip.id,
                note_id: note.id,
            })
            .await;
    }
}
