use serde::de::DeserializeOwned;
use serde::Serialize;

pub trait ApiRequest: Serialize {
    type Response: DeserializeOwned;
    const ENDPOINT: &'static str;
}
