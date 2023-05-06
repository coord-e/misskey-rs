use crate::model::{id::Id, page::Page};

use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum Request {
    #[serde(rename_all = "camelCase")]
    WithPageId { page_id: Id<Page> },
    #[serde(rename_all = "camelCase")]
    WithName { name: String, username: String },
}

impl misskey_core::Request for Request {
    type Response = Page;
    const ENDPOINT: &'static str = "pages/show";
}

#[cfg(test)]
mod tests {
    use ulid_crate::Ulid;

    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request_with_page_id() {
        let client = TestClient::new();
        let page = client
            .test(
                crate::endpoint::pages::create::Request::builder()
                    .name(Ulid::new())
                    .build(),
            )
            .await;

        client.test(Request::WithPageId { page_id: page.id }).await;
    }

    #[tokio::test]
    async fn request_with_name() {
        let client = TestClient::new();
        let user = client.me().await;
        let name = Ulid::new().to_string();
        client
            .test(
                crate::endpoint::pages::create::Request::builder()
                    .name(name.clone())
                    .build(),
            )
            .await;

        client
            .test(Request::WithName {
                name,
                username: user.username,
            })
            .await;
    }
}
