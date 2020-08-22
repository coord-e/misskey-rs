use std::{fmt::Display, str::FromStr};

use derive_more::{Display, Error, From};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(PartialEq, Eq, Clone, Debug, Copy, Display)]
pub enum SortOrder<T> {
    #[display(fmt = "+{}", _0)]
    Ascending(T),
    #[display(fmt = "-{}", _0)]
    Descending(T),
}

#[derive(Debug, Display, From, Error)]
pub enum ParseSortOrderError<E> {
    #[from(ignore)]
    #[display(fmt = "invalid sort order prefix")]
    InvalidPrefix,
    #[display(fmt = "{}", _0)]
    InvalidKey(#[error(source)] E),
}

impl<T: FromStr> FromStr for SortOrder<T> {
    type Err = ParseSortOrderError<T::Err>;

    fn from_str(s: &str) -> Result<SortOrder<T>, Self::Err> {
        if let Some(key) = s.strip_prefix('+') {
            Ok(SortOrder::Ascending(T::from_str(key)?))
        } else if let Some(key) = s.strip_prefix('-') {
            Ok(SortOrder::Descending(T::from_str(key)?))
        } else {
            Err(ParseSortOrderError::InvalidPrefix)
        }
    }
}

impl<T: Display> Serialize for SortOrder<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(self)
    }
}

impl<'de, T> Deserialize<'de> for SortOrder<T>
where
    T: FromStr,
    T::Err: Display,
{
    fn deserialize<D>(deserializer: D) -> Result<SortOrder<T>, D::Error>
    where
        D: Deserializer<'de>,
    {
        <&'de str>::deserialize(deserializer)?
            .parse()
            .map_err(serde::de::Error::custom)
    }
}
