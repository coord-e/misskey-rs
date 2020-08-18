use crate::api::ApiRequest;
use crate::model::antenna::Antenna;

use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {}

impl ApiRequest for Request {
    type Response = Vec<Antenna>;
    const ENDPOINT: &'static str = "antennas/list";
}
