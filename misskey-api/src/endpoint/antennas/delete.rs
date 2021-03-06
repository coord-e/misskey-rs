use crate::model::{antenna::Antenna, id::Id};

use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub antenna_id: Id<Antenna>,
}

impl misskey_core::Request for Request {
    type Response = ();
    const ENDPOINT: &'static str = "antennas/delete";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        use crate::model::{antenna::AntennaSource, query::Query};

        let client = TestClient::new();
        let antenna = client
            .test(crate::endpoint::antennas::create::Request {
                name: "test".to_string(),
                src: AntennaSource::All,
                user_list_id: None,
                #[cfg(feature = "12-10-0")]
                user_group_id: None,
                keywords: Query::default(),
                #[cfg(feature = "12-19-0")]
                exclude_keywords: Query::default(),
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
