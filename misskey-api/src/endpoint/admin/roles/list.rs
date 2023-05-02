use crate::model::role::Role;

use serde::Serialize;

#[derive(Serialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {}

impl misskey_core::Request for Request {
    type Response = Vec<Role>;
    const ENDPOINT: &'static str = "admin/roles/list";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let client = TestClient::new();
        client
            .admin
            .test(crate::endpoint::admin::roles::create::Request::default())
            .await;

        client.admin.test(Request::default()).await;
    }
}
