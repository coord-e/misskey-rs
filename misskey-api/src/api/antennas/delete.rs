use crate::model::antenna::AntennaId;

use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub antenna_id: AntennaId,
}

impl misskey_core::Request for Request {
    type Response = ();
    const ENDPOINT: &'static str = "antennas/delete";
}
