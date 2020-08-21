use serde::de::DeserializeOwned;
use serde::Serialize;

pub trait Request: Serialize {
    type Response: DeserializeOwned;
    const ENDPOINT: &'static str;
}

pub trait RequestWithFile: Request {}
