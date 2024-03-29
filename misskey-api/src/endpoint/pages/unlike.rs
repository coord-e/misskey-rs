use crate::model::{id::Id, page::Page};

use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub page_id: Id<Page>,
}

impl misskey_core::Request for Request {
    type Response = ();
    const ENDPOINT: &'static str = "pages/unlike";
}

#[cfg(test)]
mod tests {
    use ulid_crate::Ulid;

    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let client = TestClient::new();
        let page = client
            .admin
            .test(
                crate::endpoint::pages::create::Request::builder()
                    .name(Ulid::new())
                    .build(),
            )
            .await;

        client
            .test(crate::endpoint::pages::like::Request { page_id: page.id })
            .await;

        client.test(Request { page_id: page.id }).await;
    }
}
