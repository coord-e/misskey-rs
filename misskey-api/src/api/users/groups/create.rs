use crate::model::user_group::UserGroup;

use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    /// [ 1 .. 100 ] characters
    pub name: String,
}

impl misskey_core::Request for Request {
    type Response = UserGroup;
    const ENDPOINT: &'static str = "users/groups/create";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let mut client = TestClient::new();
        client.test(Request {
            // random 100 chars
            name: "QANQyX49AyhYTUbe8onotbllnx5VNMczPY4GBeJEuxn15aaLeoCg7RPBMrPOELdXv19vFzniwtwPsLV8QAzQ8SQJ472i9xitUyhw".to_string()
        }).await;
    }
}
