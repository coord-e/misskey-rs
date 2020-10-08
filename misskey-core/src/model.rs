//! Object types used in API.

use std::convert::Infallible;
use std::error::Error;
use std::fmt::{self, Display};
use std::str::FromStr;

use serde::{Deserialize, Serialize};

/// Trait for entity types that has an ID.
pub trait Entity {
    /// The ID type.
    type Id;
    /// Gets the ID.
    fn id(&self) -> Self::Id;
}

/// Trait for types that serves as a reference (i.e. ID) to `E`.
pub trait EntityRef<E: Entity> {
    /// Gets the reference to the entity.
    fn entity_ref(self) -> E::Id;
}

/// ID of API errors.
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Debug)]
#[serde(transparent)]
pub struct ApiErrorId(pub String);

impl FromStr for ApiErrorId {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<ApiErrorId, Infallible> {
        Ok(ApiErrorId(s.to_string()))
    }
}

impl Display for ApiErrorId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// Kind of API error.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum ApiErrorKind {
    /// The error is considered to be on the client side.
    Client,
    /// The error is considered to be on the server side.
    Server,
}

/// API error returned from Misskey.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ApiError {
    /// ID of the error.
    pub id: ApiErrorId,
    /// Human-readable description of the error.
    pub message: String,
    /// The error code, such as `INTERNAL_ERROR`.
    pub code: String,
    /// Kind of the error.
    pub kind: ApiErrorKind,
    /// Additional information on this error.
    #[serde(default)]
    pub info: serde_json::Value,
}

impl Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            ApiErrorKind::Client => write!(f, "Client error: ")?,
            ApiErrorKind::Server => write!(f, "Server error: ")?,
        }

        write!(f, "{} ({})", self.message, self.code)
    }
}

impl Error for ApiError {}

/// Result type that represents immediate response from Misskey.
///
/// [`ApiResult`] is either successful response ([`ApiResult::Ok`]) or an error
/// ([`ApiResult::Err`]) with [`ApiError`].
/// We implement this type in a way that distinguishes it from the `Result<T, ApiError>`, since the
/// [`ApiResult`] is a normal response to a successful request, even if it is an
/// [`ApiResult::Err`]. (see the return type of [`crate::Client::request`])
///
/// You can convert `ApiResult<T>` to `Result<T, ApiError>` by using [`Into::into`] or [`ApiResult::into_result`].
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
#[must_use = "this `ApiResult` may be an `Err` variant, which should be handled"]
pub enum ApiResult<T> {
    /// Contains the error value, namely [`ApiError`].
    Err {
        /// The error returned from Misskey.
        error: ApiError,
    },
    /// Contains the success value.
    Ok(T),
}

impl<T> Into<Result<T, ApiError>> for ApiResult<T> {
    /// Converts [`ApiResult`] to [`Result`] for convenient handling.
    fn into(self) -> Result<T, ApiError> {
        self.into_result()
    }
}

impl<T> ApiResult<T> {
    /// Converts [`ApiResult`] to [`Result`] for convenient handling.
    ///
    /// You can also use [`Into::into`], but this is preferred as it expresses the intent more clearly.
    pub fn into_result(self) -> Result<T, ApiError> {
        match self {
            ApiResult::Err { error } => Err(error),
            ApiResult::Ok(x) => Ok(x),
        }
    }

    /// Converts [`ApiResult<T>`][`ApiResult`] to [`Option<T>`][`Option`], consuming `self`, and discarding the error, if any.
    pub fn ok(self) -> Option<T> {
        match self {
            ApiResult::Err { .. } => None,
            ApiResult::Ok(x) => Some(x),
        }
    }

    /// Converts [`ApiResult<T>`][`ApiResult`] to [`Option<ApiError>`][`Option`], consuming `self`, and discarding the success
    /// value, if any.
    pub fn err(self) -> Option<ApiError> {
        match self {
            ApiResult::Err { error } => Some(error),
            ApiResult::Ok(_) => None,
        }
    }

    /// Returns true if the API result is [`ApiResult::Ok`].
    pub fn is_ok(&self) -> bool {
        match self {
            ApiResult::Err { .. } => false,
            ApiResult::Ok(_) => true,
        }
    }

    /// Returns true if the API result is [`ApiResult::Err`].
    pub fn is_err(&self) -> bool {
        !self.is_ok()
    }

    /// Returns the contained [`ApiResult::Ok`] value, consuming the `self` value.
    ///
    /// # Panics
    ///
    /// Panics if the value is an [`ApiResult::Err`], with a panic message including the
    /// passed message, and the content of the [`ApiResult::Err`].
    pub fn expect(self, msg: &str) -> T {
        match self {
            ApiResult::Err { error } => panic!("{}: {:?}", msg, error),
            ApiResult::Ok(x) => x,
        }
    }

    /// Returns the contained [`ApiResult::Ok`] value, consuming the `self` value.
    ///
    /// # Panics
    ///
    /// Panics if the value is an [`ApiResult::Err`], with a panic message provided by the
    /// [`ApiResult::Err`]'s value.
    pub fn unwrap(self) -> T {
        self.expect("called `ApiResult::unwrap()` on an `ApiResult::Err` value")
    }

    /// Maps a [`ApiResult<T>`][`ApiResult`] to [`ApiResult<U>`][`ApiResult`] by applying a function to a
    /// contained [`ApiResult::Ok`] value, leaving an [`ApiResult::Err`] value untouched.
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
