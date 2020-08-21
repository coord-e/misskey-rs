use crate::model::antenna::Antenna;

use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {}

impl misskey_core::Request for Request {
    type Response = Vec<Antenna>;
    const ENDPOINT: &'static str = "antennas/list";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        use crate::model::antenna::AntennaSource;

        let mut client = TestClient::new();
        client
            .test(crate::api::antennas::create::Request {
                name: "test".to_string(),
                src: AntennaSource::All,
                user_list_id: None,
                user_group_id: None,
                keywords: Vec::new(),
                exclude_keywords: None,
                users: Vec::new(),
                case_sensitive: false,
                with_replies: false,
                with_file: false,
                notify: false,
            })
            .await;

        client.test(Request {}).await;
    }
}
