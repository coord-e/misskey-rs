use crate::model::antenna::{Antenna, AntennaId};

use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub antenna_id: AntennaId,
}

impl misskey_core::Request for Request {
    type Response = Antenna;
    const ENDPOINT: &'static str = "antennas/show";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        use crate::model::antenna::AntennaSource;

        let mut client = TestClient::new();
        let antenna = client
            .user
            .test(crate::api::antennas::create::Request {
                name: "test".to_string(),
                src: AntennaSource::All,
                user_list_id: None,
                #[cfg(feature = "12-10-0")]
                user_group_id: None,
                keywords: vec![vec!["hello".to_string(), "awesome".to_string()]],
                #[cfg(feature = "12-19-0")]
                exclude_keywords: vec![vec!["oh".to_string()]],
                users: Vec::new(),
                case_sensitive: false,
                with_replies: false,
                with_file: false,
                notify: false,
            })
            .await;

        client
            .test(Request {
                antenna_id: antenna.id,
            })
            .await;
    }
}
