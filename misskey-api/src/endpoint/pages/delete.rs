use crate::model::{id::Id, page::Page};

use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub page_id: Id<Page>,
}

impl misskey_core::Request for Request {
    type Response = ();
    const ENDPOINT: &'static str = "pages/delete";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let client = TestClient::new();
        let page = client
            .test(crate::endpoint::pages::create::Request::default())
            .await;

        client.test(Request { page_id: page.id }).await;
    }
}
