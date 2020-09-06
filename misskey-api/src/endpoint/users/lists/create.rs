use crate::model::user_list::UserList;

use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    /// [ 1 .. 100 ] characters
    pub name: String,
}

impl misskey_core::Request for Request {
    type Response = UserList;
    const ENDPOINT: &'static str = "users/lists/create";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let client = TestClient::new();
        client.test(Request {
            // random 100 chars
            name: "PCr35G4uQmQWU5Gypsg14cayVRtO5LHRnQ8EWxTeWo7RxrIIzDrsNDricm6u2wEdvwPEPU3CTIlmovhB98Gv4FwZYHXZEDGTDiy7".to_string()
        }).await;
    }
}
