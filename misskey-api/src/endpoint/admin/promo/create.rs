use crate::model::{id::Id, note::Note};

use chrono::{serde::ts_milliseconds, DateTime, Utc};
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub note_id: Id<Note>,
    #[serde(with = "ts_milliseconds")]
    pub expires_at: DateTime<Utc>,
}

impl misskey_core::Request for Request {
    type Response = ();
    const ENDPOINT: &'static str = "admin/promo/create";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let client = TestClient::new();
        let note = client.create_note(Some("hi"), None, None).await;

        client
            .admin
            .test(Request {
                note_id: note.id,
                expires_at: chrono::Utc::now() + chrono::Duration::hours(1),
            })
            .await;
    }
}
