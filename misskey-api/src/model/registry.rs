use std::borrow::Cow;
use std::fmt::{self, Display};
use std::str::FromStr;

use serde::{
    de::{self, Deserializer},
    ser::{SerializeSeq, Serializer},
    Deserialize, Serialize,
};
use std::mem::Discriminant;
use thiserror::Error;

/// An error returned by [`RegistryScope::from_segments`].
#[derive(Debug, Error, Clone)]
#[error("invalid registry scope segment")]
pub struct FromSegmentsError {
    _priv: (),
}

/// A "scope" in registry.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RegistryScope(Vec<String>);

// The method names and signatures have been carefully chosen to leave room to add a non-allocating
// 'static variant internally when `const_panic` is stabilized to enable `RegistryScope` constant.
impl RegistryScope {
    /// Creates [`RegistryScope`] from the sequence of segments.
    ///
    /// Returns an error if the input does not match `^[a-zA-Z0-9_]$`.
    pub fn from_segments<I, T>(segments: I) -> Result<Self, FromSegmentsError>
    where
        I: IntoIterator<Item = T>,
        T: Into<String>,
    {
        let segments: Vec<_> = segments.into_iter().map(Into::into).collect();
        if segments.iter().all(|s| is_valid_segment(s)) {
            Ok(RegistryScope(segments))
        } else {
            Err(FromSegmentsError { _priv: () })
        }
    }

    /// Turns [`RegistryScope`] into a vector of segments.
    pub fn into_segments(self) -> Vec<String> {
        self.0
    }

    /// Returns an iterator over the segments in this [`RegistryScope`].
    pub fn segments(&self) -> impl ExactSizeIterator + Iterator<Item = &str> {
        self.0.iter().map(AsRef::as_ref)
    }
}

macro_rules! impl_partial_eq {
    ($base:ty, $t:ty, $other:ty $(, #[$m:meta])?) => {
        $(#[$m])?
        impl PartialEq<$other> for $t {
            fn eq(&self, other: &$other) -> bool {
                // OK because of symmetricity
                // (we don't have PartialEq<Cow<'_, [A]>> for Vec<B>)
                <$other as PartialEq<$base>>::eq(other, &self.0)
            }
        }
        $(#[$m])?
        impl PartialEq<$t> for $other {
            fn eq(&self, other: &$t) -> bool {
                <$other as PartialEq<$base>>::eq(self, &other.0)
            }
        }
    };
}

impl_partial_eq! { Vec<String>, RegistryScope, Vec<&str> }
impl_partial_eq! { Vec<String>, RegistryScope, [&str], #[rustversion::since(1.49)] }
impl_partial_eq! { Vec<String>, RegistryScope, &[&str], #[rustversion::since(1.46)] }
impl_partial_eq! { Vec<String>, RegistryScope, &mut [&str], #[rustversion::since(1.46)] }
impl_partial_eq! { Vec<String>, RegistryScope, Cow<'_, [&str]> }
impl_partial_eq! { Vec<String>, RegistryScope, Vec<String> }
impl_partial_eq! { Vec<String>, RegistryScope, [String], #[rustversion::since(1.49)] }
impl_partial_eq! { Vec<String>, RegistryScope, &[String], #[rustversion::since(1.46)] }
impl_partial_eq! { Vec<String>, RegistryScope, &mut [String], #[rustversion::since(1.46)] }
impl_partial_eq! { Vec<String>, RegistryScope, Cow<'_, [String]> }

impl<'de> Deserialize<'de> for RegistryScope {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        RegistryScope::from_segments(Vec::<String>::deserialize(deserializer)?)
            .map_err(de::Error::custom)
    }
}

impl Serialize for RegistryScope {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let iter = self.segments();
        let mut seq = serializer.serialize_seq(Some(iter.len()))?;
        for segment in iter {
            seq.serialize_element(segment)?;
        }
        seq.end()
    }
}

fn is_valid_segment(segment: &str) -> bool {
    segment
        .bytes()
        .all(|b| matches!(b, b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'_'))
}

/// A key in registry.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(transparent)]
pub struct RegistryKey(pub String);

impl Display for RegistryKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl FromStr for RegistryKey {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<RegistryKey, Self::Err> {
        Ok(RegistryKey(s.to_string()))
    }
}

impl<S: Into<String>> From<S> for RegistryKey {
    fn from(s: S) -> RegistryKey {
        RegistryKey(s.into())
    }
}

impl_partial_eq! { String, RegistryKey, String }
impl_partial_eq! { String, RegistryKey, str }
impl_partial_eq! { String, RegistryKey, &str }
impl_partial_eq! { String, RegistryKey, Cow<'_, str> }

/// A value in registry is simply the JSON value.
pub type RegistryValue = serde_json::Value;

/// An error which can be returned when parsing [`RegistryValueType`].
#[derive(Debug, Error, Clone)]
#[error("invalid registry value type")]
pub struct ParseRegistryValueTypeError {
    _priv: (),
}

/// A type of [`RegistryValue`].
///
/// This is effectively equivalent to `Discriminant<RegistryValue>`.
/// Implemented here separately for convenience (especially in deserialization), still they can be
/// converted to each other.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(rename_all = "camelCase")]
pub enum RegistryValueType {
    /// A type of [`Value::Null`][`serde_json::Value::Null`].
    Null,
    /// A type of [`Value::Bool`][`serde_json::Value::Bool`].
    Boolean,
    /// A type of [`Value::Number`][`serde_json::Value::Number`].
    Number,
    /// A type of [`Value::String`][`serde_json::Value::String`].
    String,
    /// A type of [`Value::Array`][`serde_json::Value::Array`].
    Array,
    /// A type of [`Value::Object`][`serde_json::Value::Object`].
    Object,
}

impl RegistryValueType {
    /// Returns [`RegistryValueType`] corresponding to the `value`.
    pub fn of(value: &RegistryValue) -> Self {
        use serde_json::Value;
        match value {
            Value::Null => RegistryValueType::Null,
            Value::Bool(_) => RegistryValueType::Boolean,
            Value::Number(_) => RegistryValueType::Number,
            Value::String(_) => RegistryValueType::String,
            Value::Array(_) => RegistryValueType::Array,
            Value::Object(_) => RegistryValueType::Object,
        }
    }
}

fn discriminant_of<T: Default>(ctor: fn(T) -> RegistryValue) -> Discriminant<RegistryValue> {
    std::mem::discriminant(&ctor(Default::default()))
}

impl From<Discriminant<RegistryValue>> for RegistryValueType {
    fn from(discriminant: Discriminant<RegistryValue>) -> Self {
        use serde_json::Value;
        if discriminant == std::mem::discriminant(&Value::Null) {
            RegistryValueType::Null
        } else if discriminant == discriminant_of(Value::Bool) {
            RegistryValueType::Boolean
        } else if discriminant == std::mem::discriminant(&Value::Number(0_u64.into())) {
            RegistryValueType::Number
        } else if discriminant == discriminant_of(Value::String) {
            RegistryValueType::String
        } else if discriminant == discriminant_of(Value::Array) {
            RegistryValueType::Array
        } else if discriminant == discriminant_of(Value::Object) {
            RegistryValueType::Object
        } else {
            unreachable!();
        }
    }
}

impl From<RegistryValueType> for Discriminant<RegistryValue> {
    fn from(ty: RegistryValueType) -> Self {
        use serde_json::Value;
        match ty {
            RegistryValueType::Null => std::mem::discriminant(&Value::Null),
            RegistryValueType::Boolean => discriminant_of(Value::Bool),
            RegistryValueType::Number => std::mem::discriminant(&Value::Number(0_u64.into())),
            RegistryValueType::String => discriminant_of(Value::String),
            RegistryValueType::Array => discriminant_of(Value::Array),
            RegistryValueType::Object => discriminant_of(Value::Object),
        }
    }
}

impl PartialEq<Discriminant<RegistryValue>> for RegistryValueType {
    fn eq(&self, other: &Discriminant<RegistryValue>) -> bool {
        Discriminant::<RegistryValue>::from(*self) == *other
    }
}

impl PartialEq<RegistryValueType> for Discriminant<RegistryValue> {
    fn eq(&self, other: &RegistryValueType) -> bool {
        *self == Discriminant::<RegistryValue>::from(*other)
    }
}

impl Display for RegistryValueType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RegistryValueType::Null => f.write_str("null"),
            RegistryValueType::Boolean => f.write_str("boolean"),
            RegistryValueType::Number => f.write_str("number"),
            RegistryValueType::String => f.write_str("string"),
            RegistryValueType::Array => f.write_str("array"),
            RegistryValueType::Object => f.write_str("object"),
        }
    }
}

impl FromStr for RegistryValueType {
    type Err = ParseRegistryValueTypeError;
    fn from_str(s: &str) -> Result<RegistryValueType, Self::Err> {
        match s {
            "null" | "Null" => Ok(RegistryValueType::Null),
            "boolean" | "Boolean" => Ok(RegistryValueType::Boolean),
            "number" | "Number" => Ok(RegistryValueType::Number),
            "string" | "String" => Ok(RegistryValueType::String),
            "array" | "Array" => Ok(RegistryValueType::Array),
            "object" | "Object" => Ok(RegistryValueType::Array),
            _ => Err(ParseRegistryValueTypeError { _priv: () }),
        }
    }
}
