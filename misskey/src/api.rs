use serde::de::DeserializeOwned;
use serde::Serialize;

pub mod admin;
pub mod antennas;
pub mod blocking;
pub mod charts;
pub mod following;
pub mod messaging;
pub mod mute;
pub mod notes;
pub mod users;

pub trait ApiRequest: Serialize {
    type Response: DeserializeOwned;
    const ENDPOINT: &'static str;
}
