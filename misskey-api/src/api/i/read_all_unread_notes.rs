use misskey_core::ApiRequest;
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {}

impl ApiRequest for Request {
    type Response = ();
    const ENDPOINT: &'static str = "i/read-all-unread-notes";
}
