use serde::de::DeserializeOwned;
use serde::Serialize;

pub mod admin;
pub mod charts;
pub mod following;
pub mod messaging;
pub mod notes;

pub trait ApiRequest: Serialize {
    type Response: DeserializeOwned;
    const ENDPOINT: &'static str;
}
