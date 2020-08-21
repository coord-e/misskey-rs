use crate::model::{
    antenna::{Antenna, AntennaId, AntennaSource},
    user_group::UserGroupId,
    user_list::UserListId,
};

use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub antenna_id: AntennaId,
    /// [ 1 .. 100 ] characters
    pub name: String,
    pub src: AntennaSource,
    pub user_list_id: Option<UserListId>,
    pub user_group_id: Option<UserGroupId>,
    pub keywords: Vec<Vec<String>>,
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
