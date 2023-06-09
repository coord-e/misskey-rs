#[cfg(feature = "12-10-0")]
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
    pub antenna_id: Id<Antenna>,
    /// [ 1 .. 100 ] characters
    #[builder(setter(into))]
    pub name: String,
    pub src: AntennaSource,
    #[builder(default, setter(strip_option))]
    pub user_list_id: Option<Id<UserList>>,
    #[cfg(feature = "12-10-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-10-0")))]
    #[builder(default, setter(strip_option))]
    pub user_group_id: Option<Id<UserGroup>>,
    #[builder(default, setter(into))]
    pub keywords: Query<String>,
    #[cfg(feature = "12-19-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-19-0")))]
    #[builder(default, setter(into))]
    pub exclude_keywords: Query<String>,
    pub users: Vec<String>,
    pub case_sensitive: bool,
    pub with_replies: bool,
    pub with_file: bool,
    pub notify: bool,
}

impl misskey_core::Request for Request {
    type Response = Antenna;
    const ENDPOINT: &'static str = "antennas/update";
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
            .user
            .test(
                crate::endpoint::antennas::create::Request::builder()
                    .name("test")
                    .keywords(Query::from_vec(vec![vec![
                        "hello".to_string(),
                        "awesome".to_string(),
                    ]]))
                    .case_sensitive(true)
                    .with_file(true)
                    .build(),
            )
            .await;

        let list = client
            .test(crate::endpoint::users::lists::create::Request {
                name: "test".to_string(),
            })
            .await;

        client
            .test(Request {
                antenna_id: antenna.id,
                name: "test2".to_string(),
                src: AntennaSource::List,
                user_list_id: Some(list.id),
                #[cfg(feature = "12-10-0")]
                user_group_id: None,
                keywords: Query::from_vec(vec![vec!["cool".to_string()], vec!["nice".to_string()]]),
                #[cfg(feature = "12-19-0")]
                exclude_keywords: Query::default(),
                users: Vec::new(),
                case_sensitive: false,
                with_replies: true,
                with_file: false,
                notify: true,
            })
            .await;
    }
}
