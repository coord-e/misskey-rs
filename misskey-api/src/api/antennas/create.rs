#[cfg(feature = "12-10-0")]
use crate::model::user_group::UserGroupId;
use crate::model::{
    antenna::{Antenna, AntennaSource},
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
    #[cfg(feature = "12-10-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-10-0")))]
    pub user_group_id: Option<UserGroupId>,
    pub keywords: Vec<Vec<String>>,
    #[cfg(feature = "12-19-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-19-0")))]
    pub exclude_keywords: Vec<Vec<String>>,
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
                #[cfg(feature = "12-10-0")]
                user_group_id: None,
                keywords: vec![vec!["hello".to_string(), "awesome".to_string()]],
                #[cfg(feature = "12-19-0")]
                exclude_keywords: vec![vec!["no".to_string()]],
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
                #[cfg(feature = "12-10-0")]
                user_group_id: None,
                keywords: vec![vec!["hey".to_string()], vec!["wow".to_string()]],
                #[cfg(feature = "12-19-0")]
                exclude_keywords: Vec::new(),
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
                #[cfg(feature = "12-10-0")]
                user_group_id: None,
                keywords: vec![vec!["kawaii".to_string()], vec!["cute".to_string()]],
                #[cfg(feature = "12-19-0")]
                exclude_keywords: vec![vec!["sensitive".to_string()], vec!["violence".to_string()]],
                users: Vec::new(),
                case_sensitive: false,
                with_replies: true,
                with_file: true,
                notify: false,
            })
            .await;
    }

    #[tokio::test]
    #[cfg(feature = "12-10-0")]
    async fn request_group() {
        use crate::model::antenna::AntennaSource;

        let mut client = TestClient::new();
        let group = client
            .test(crate::api::users::groups::create::Request {
                name: "test".to_string(),
            })
            .await;

        client
            .test(Request {
                name: "test".to_string(),
                src: AntennaSource::Group,
                user_list_id: None,
                user_group_id: Some(group.id),
                keywords: vec![vec!["kawaii".to_string()], vec!["cute".to_string()]],
                #[cfg(feature = "12-19-0")]
                exclude_keywords: Vec::new(),
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
                #[cfg(feature = "12-10-0")]
                user_group_id: None,
                keywords: vec![vec!["annoucement".to_string()], vec!["notice".to_string()]],
                #[cfg(feature = "12-19-0")]
                exclude_keywords: Vec::new(),
                users: vec![admin.username],
                case_sensitive: false,
                with_replies: true,
                with_file: false,
                notify: true,
            })
            .await;
    }
}
