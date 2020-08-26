use crate::model::user_list::UserList;

use serde::Serialize;

#[derive(Serialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {}

impl misskey_core::Request for Request {
    type Response = Vec<UserList>;
    const ENDPOINT: &'static str = "users/lists/list";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let mut client = TestClient::new();
        client
            .test(crate::api::users::lists::create::Request {
                name: "test".to_string(),
            })
            .await;

        client.test(Request::default()).await;
    }
}
