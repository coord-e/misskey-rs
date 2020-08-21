use crate::model::antenna::Antenna;

use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {}

impl misskey_core::Request for Request {
    type Response = Vec<Antenna>;
    const ENDPOINT: &'static str = "antennas/list";
}
