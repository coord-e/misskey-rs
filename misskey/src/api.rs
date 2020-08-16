use serde::de::DeserializeOwned;
use serde::Serialize;

pub mod following;
pub mod messaging;
pub mod notes;

pub trait ApiRequest: Serialize {
    type Response: DeserializeOwned;
    const ENDPOINT: &'static str;
}
