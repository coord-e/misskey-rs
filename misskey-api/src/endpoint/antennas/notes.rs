use crate::model::{antenna::Antenna, id::Id, note::Note};

use serde::Serialize;
use typed_builder::TypedBuilder;

#[derive(Serialize, Debug, Clone, TypedBuilder)]
#[serde(rename_all = "camelCase")]
#[builder(doc)]
pub struct Request {
    pub antenna_id: Id<Antenna>,
    /// 1 .. 100
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub limit: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub since_id: Option<Id<Note>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub until_id: Option<Id<Note>>,
}

impl misskey_core::Request for Request {
    type Response = Vec<Note>;
    const ENDPOINT: &'static str = "antennas/notes";
}

impl_pagination!(Request, Note);

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        use crate::model::antenna::AntennaSource;

        let client = TestClient::new();
        let antenna = client
            .user
            .test(crate::endpoint::antennas::create::Request {
                name: "test".to_string(),
                src: AntennaSource::All,
                user_list_id: None,
                #[cfg(feature = "12-10-0")]
                user_group_id: None,
                keywords: Vec::new(),
                #[cfg(feature = "12-19-0")]
                exclude_keywords: Vec::new(),
                users: Vec::new(),
                case_sensitive: false,
                with_replies: false,
                with_file: false,
                notify: false,
            })
            .await;

        client
            .user
            .test(Request {
                antenna_id: antenna.id,
                limit: None,
                since_id: None,
                until_id: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_with_limit() {
        use crate::model::antenna::AntennaSource;

        let client = TestClient::new();
        let antenna = client
            .user
            .test(crate::endpoint::antennas::create::Request {
                name: "test".to_string(),
                src: AntennaSource::All,
                user_list_id: None,
                #[cfg(feature = "12-10-0")]
                user_group_id: None,
                keywords: vec![vec!["hello".to_string(), "awesome".to_string()]],
                #[cfg(feature = "12-19-0")]
                exclude_keywords: Vec::new(),
                users: Vec::new(),
                case_sensitive: false,
                with_replies: false,
                with_file: false,
                notify: false,
            })
            .await;

        client
            .user
            .test(Request {
                antenna_id: antenna.id,
                limit: Some(100),
                since_id: None,
                until_id: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_paginate() {
        use crate::model::antenna::AntennaSource;

        let client = TestClient::new();
        let antenna = client
            .user
            .test(crate::endpoint::antennas::create::Request {
                name: "test".to_string(),
                src: AntennaSource::All,
                user_list_id: None,
                #[cfg(feature = "12-10-0")]
                user_group_id: None,
                keywords: vec![vec!["hello".to_string(), "awesome".to_string()]],
                #[cfg(feature = "12-19-0")]
                exclude_keywords: Vec::new(),
                users: Vec::new(),
                case_sensitive: false,
                with_replies: false,
                with_file: false,
                notify: false,
            })
            .await;
        let note = client
            .admin
            .create_note(Some("hello, world"), None, None)
            .await;

        client
            .user
            .test(Request {
                antenna_id: antenna.id,
                limit: None,
                since_id: Some(note.id.clone()),
                until_id: Some(note.id.clone()),
            })
            .await;
    }
}
