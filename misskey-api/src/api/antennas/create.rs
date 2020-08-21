use crate::model::{
    antenna::{Antenna, AntennaSource},
    user_group::UserGroupId,
    user_list::UserListId,
};

use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    /// [ 1 .. 100 ] characters
    pub name: String,
    pub src: AntennaSource,
    pub user_list_id: Option<UserListId>,
    /// not available in <12.10.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_group_id: Option<UserGroupId>,
    pub keywords: Vec<Vec<String>>,
    /// not available in <12.19.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_keywords: Option<Vec<Vec<String>>>,
    pub users: Vec<String>,
    pub case_sensitive: bool,
    pub with_replies: bool,
    pub with_file: bool,
    pub notify: bool,
}

impl misskey_core::Request for Request {
    type Response = Antenna;
    const ENDPOINT: &'static str = "antennas/create";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request_all() {
        use crate::model::antenna::AntennaSource;

        let mut client = TestClient::new();
        client
            .test(Request {
                // random 100 chars
                name: "z0LnEV7NljIUEFFBkjTMW7BN2f6GhfnkbjrNWTqsPikqBzbd02jAvN1axE9h9ZyYCIklKt4WIeeyCNxB31TxJW6hJyHAJVnjTPJC".to_string(),
                src: AntennaSource::All,
                user_list_id: None,
                user_group_id: None,
                keywords: vec![vec!["hello".to_string(), "awesome".to_string()]],
                exclude_keywords: None,
                users: Vec::new(),
                case_sensitive: false,
                with_replies: false,
                with_file: false,
                notify: false,
            })
            .await;
    }

    #[tokio::test]
    async fn request_home() {
        use crate::model::antenna::AntennaSource;

        let mut client = TestClient::new();
        client
            .test(Request {
                name: "test".to_string(),
                src: AntennaSource::Home,
                user_list_id: None,
                user_group_id: None,
                keywords: vec![vec!["hey".to_string()], vec!["wow".to_string()]],
                exclude_keywords: None,
                users: Vec::new(),
                case_sensitive: true,
                with_replies: false,
                with_file: false,
                notify: false,
            })
            .await;
    }

    #[tokio::test]
    async fn request_list() {
        use crate::model::antenna::AntennaSource;

        let mut client = TestClient::new();
        let list = client
            .test(crate::api::users::lists::create::Request {
                name: "test".to_string(),
            })
            .await;

        client
            .test(Request {
                name: "test".to_string(),
                src: AntennaSource::List,
                user_list_id: Some(list.id),
                user_group_id: None,
                keywords: vec![vec!["kawaii".to_string()], vec!["cute".to_string()]],
                exclude_keywords: None,
                users: Vec::new(),
                case_sensitive: false,
                with_replies: true,
                with_file: true,
                notify: false,
            })
            .await;
    }

    #[tokio::test]
    async fn request_users() {
        use crate::model::antenna::AntennaSource;

        let mut client = TestClient::new();
        let admin = client.admin.me().await;

        client
            .user
            .test(Request {
                name: "test".to_string(),
                src: AntennaSource::Users,
                user_list_id: None,
                user_group_id: None,
                keywords: vec![vec!["annoucement".to_string()], vec!["notice".to_string()]],
                exclude_keywords: None,
                users: vec![admin.username],
                case_sensitive: false,
                with_replies: true,
                with_file: false,
                notify: true,
            })
            .await;
    }
}
