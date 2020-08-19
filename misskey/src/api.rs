use derive_more::{Display, FromStr};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

pub mod admin;
pub mod antennas;
pub mod blocking;
pub mod charts;
pub mod following;
pub mod i;
pub mod messaging;
pub mod mute;
pub mod notes;
pub mod users;

pub trait ApiRequest: Serialize {
    type Response: DeserializeOwned;
    const ENDPOINT: &'static str;
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, FromStr, Debug, Display)]
#[serde(transparent)]
pub struct ApiErrorId(pub String);

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum ApiErrorKind {
    Client,
    Server,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ApiError {
    pub id: ApiErrorId,
    pub message: String,
    pub code: String,
    pub kind: ApiErrorKind,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
#[must_use = "this `ApiResult` may be an `Err` variant, which should be handled"]
pub enum ApiResult<T> {
    Err { error: ApiError },
    Ok(T),
}

impl<T> Into<Result<T, ApiError>> for ApiResult<T> {
    fn into(self) -> Result<T, ApiError> {
        self.into_result()
    }
}

impl<T> ApiResult<T> {
    pub fn into_result(self) -> Result<T, ApiError> {
        match self {
            ApiResult::Err { error } => Err(error),
            ApiResult::Ok(x) => Ok(x),
        }
    }

    pub fn ok(self) -> Option<T> {
        match self {
            ApiResult::Err { .. } => None,
            ApiResult::Ok(x) => Some(x),
        }
    }

    pub fn err(self) -> Option<ApiError> {
        match self {
            ApiResult::Err { error } => Some(error),
            ApiResult::Ok(_) => None,
        }
    }

    pub fn is_ok(&self) -> bool {
        match self {
            ApiResult::Err { .. } => false,
            ApiResult::Ok(_) => true,
        }
    }

    pub fn is_err(&self) -> bool {
        !self.is_ok()
    }

    pub fn expect(self, msg: &str) -> T {
        match self {
            ApiResult::Err { error } => panic!("{}: {:?}", msg, error),
            ApiResult::Ok(x) => x,
        }
    }

    pub fn unwrap(self) -> T {
        self.expect("called `ApiResult::unwrap()` on an `ApiResult::Err` value")
    }

    pub fn map<U, F>(self, op: F) -> ApiResult<U>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            ApiResult::Ok(x) => ApiResult::Ok(op(x)),
            ApiResult::Err { error } => ApiResult::Err { error },
        }
    }
}
