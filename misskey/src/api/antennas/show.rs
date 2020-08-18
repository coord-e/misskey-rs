use crate::api::ApiRequest;
use crate::model::antenna::{Antenna, AntennaId};

use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub antenna_id: AntennaId,
}

impl ApiRequest for Request {
    type Response = Antenna;
    const ENDPOINT: &'static str = "antennas/show";
}
