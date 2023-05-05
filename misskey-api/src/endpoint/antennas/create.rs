#[cfg(all(feature = "12-10-0", not(feature = "13-7-0")))]
use crate::model::user_group::UserGroup;
use crate::model::{
    antenna::{Antenna, AntennaSource},
    id::Id,
    query::Query,
    user_list::UserList,
};

use serde::Serialize;
use typed_builder::TypedBuilder;

#[derive(Serialize, Debug, Clone, TypedBuilder)]
#[serde(rename_all = "camelCase")]
#[builder(doc)]
pub struct Request {
    /// [ 1 .. 100 ] characters
    #[builder(setter(into))]
    pub name: String,
    #[builder(default)]
    pub src: AntennaSource,
    #[builder(default, setter(strip_option))]
    pub user_list_id: Option<Id<UserList>>,
    #[cfg(all(feature = "12-10-0", not(feature = "13-7-0")))]
    #[cfg_attr(docsrs, doc(cfg(all(feature = "12-10-0", not(feature = "13-7-0")))))]
    #[builder(default, setter(strip_option))]
    pub user_group_id: Option<Id<UserGroup>>,
    #[cfg_attr(not(feature = "13-10-0"), builder(default))]
    #[builder(setter(into))]
    pub keywords: Query<String>,
    #[cfg(feature = "12-19-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-19-0")))]
    #[builder(default, setter(into))]
    pub exclude_keywords: Query<String>,
    #[builder(default)]
    pub users: Vec<String>,
    #[builder(default)]
    pub case_sensitive: bool,
    #[builder(default)]
    pub with_replies: bool,
    #[builder(default)]
    pub with_file: bool,
    #[builder(default)]
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
        use crate::model::{antenna::AntennaSource, query::Query};

        let client = TestClient::new();
        client
            .test(Request {
                // random 100 chars
                name: "z0LnEV7NljIUEFFBkjTMW7BN2f6GhfnkbjrNWTqsPikqBzbd02jAvN1axE9h9ZyYCIklKt4WIeeyCNxB31TxJW6hJyHAJVnjTPJC".to_string(),
                src: AntennaSource::All,
                user_list_id: None,
                #[cfg(all(feature = "12-10-0", not(feature = "13-7-0")))]
                user_group_id: None,
                keywords: Query::from_vec(vec![vec!["hello".to_string(), "awesome".to_string()]]),
                #[cfg(feature = "12-19-0")]
                exclude_keywords: Query::from_vec(vec![vec!["no".to_string()]]),
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
        use crate::model::{antenna::AntennaSource, query::Query};

        let client = TestClient::new();
        client
            .test(Request {
                name: "test".to_string(),
                src: AntennaSource::Home,
                user_list_id: None,
                #[cfg(all(feature = "12-10-0", not(feature = "13-7-0")))]
                user_group_id: None,
                keywords: Query::from_vec(vec![vec!["hey".to_string()], vec!["wow".to_string()]]),
                #[cfg(feature = "12-19-0")]
                exclude_keywords: Query::default(),
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
        use crate::model::{antenna::AntennaSource, query::Query};

        let client = TestClient::new();
        let list = client
            .test(crate::endpoint::users::lists::create::Request {
                name: "test".to_string(),
            })
            .await;

        client
            .test(Request {
                name: "test".to_string(),
                src: AntennaSource::List,
                user_list_id: Some(list.id),
                #[cfg(all(feature = "12-10-0", not(feature = "13-7-0")))]
                user_group_id: None,
                keywords: Query::from_vec(vec![
                    vec!["kawaii".to_string()],
                    vec!["cute".to_string()],
                ]),
                #[cfg(feature = "12-19-0")]
                exclude_keywords: Query::from_vec(vec![
                    vec!["sensitive".to_string()],
                    vec!["violence".to_string()],
                ]),
                users: Vec::new(),
                case_sensitive: false,
                with_replies: true,
                with_file: true,
                notify: false,
            })
            .await;
    }

    #[tokio::test]
    #[cfg(all(feature = "12-10-0", not(feature = "13-7-0")))]
    async fn request_group() {
        use crate::model::{antenna::AntennaSource, query::Query};

        let client = TestClient::new();
        let group = client
            .test(crate::endpoint::users::groups::create::Request {
                name: "test".to_string(),
            })
            .await;

        client
            .test(Request {
                name: "test".to_string(),
                src: AntennaSource::Group,
                user_list_id: None,
                user_group_id: Some(group.id),
                keywords: Query::from_vec(vec![
                    vec!["kawaii".to_string()],
                    vec!["cute".to_string()],
                ]),
                #[cfg(feature = "12-19-0")]
                exclude_keywords: Query::default(),
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
        use crate::model::{antenna::AntennaSource, query::Query};

        let client = TestClient::new();
        let admin = client.admin.me().await;

        client
            .user
            .test(Request {
                name: "test".to_string(),
                src: AntennaSource::Users,
                user_list_id: None,
                #[cfg(all(feature = "12-10-0", not(feature = "13-7-0")))]
                user_group_id: None,
                keywords: Query::from_vec(vec![
                    vec!["annoucement".to_string()],
                    vec!["notice".to_string()],
                ]),
                #[cfg(feature = "12-19-0")]
                exclude_keywords: Query::default(),
                users: vec![admin.username],
                case_sensitive: false,
                with_replies: true,
                with_file: false,
                notify: true,
            })
            .await;
    }
}
