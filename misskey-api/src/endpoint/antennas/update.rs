#[cfg(feature = "12-10-0")]
use crate::model::user_group::UserGroup;
use crate::model::{
    antenna::{Antenna, AntennaSource},
    id::Id,
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
    const ENDPOINT: &'static str = "antennas/update";
}

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
                keywords: vec![vec!["hello".to_string(), "awesome".to_string()]],
                #[cfg(feature = "12-19-0")]
                exclude_keywords: Vec::new(),
                users: Vec::new(),
                case_sensitive: true,
                with_replies: false,
                with_file: true,
                notify: false,
            })
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
                keywords: vec![vec!["cool".to_string()], vec!["nice".to_string()]],
                #[cfg(feature = "12-19-0")]
                exclude_keywords: Vec::new(),
                users: Vec::new(),
                case_sensitive: false,
                with_replies: true,
                with_file: false,
                notify: true,
            })
            .await;
    }
}
