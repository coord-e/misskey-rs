use crate::model::drive::DriveFile;
use crate::model::id::Id;

use serde::Serialize;
use typed_builder::TypedBuilder;
#[cfg(any(docsrs, not(feature = "12-9-0")))]
use url::Url;

#[derive(Serialize, Debug, Clone, TypedBuilder)]
#[serde(rename_all = "camelCase")]
#[builder(doc)]
pub struct Request {
    pub file_id: Id<DriveFile>,
}

impl misskey_core::Request for Request {
    type Response = ();
    const ENDPOINT: &'static str = "admin/emoji/import-zip";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let client = TestClient::new();
        let image_url = client.avatar_url().await;
        let file = client.upload_from_url(image_url).await;
        client.admin.test(Request { file_id: file.id }).await;
    }
}
