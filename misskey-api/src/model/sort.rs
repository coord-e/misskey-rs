use std::fmt::{self, Debug, Display};
use std::str::FromStr;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(PartialEq, Eq, Clone, Debug, Copy)]
pub enum SortOrder<T> {
    Ascending(T),
    Descending(T),
}

impl<T: Display> Display for SortOrder<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SortOrder::Ascending(k) => write!(f, "+{}", k),
            SortOrder::Descending(k) => write!(f, "-{}", k),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ParseSortOrderError<E> {
    _priv: InternalParseSortOrderError<E>,
}

impl<E> std::error::Error for ParseSortOrderError<E>
where
    E: std::error::Error + 'static,
{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self._priv.source()
    }
}

impl<E> Display for ParseSortOrderError<E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self._priv.fmt(f)
    }
}

#[derive(Debug, Clone)]
enum InternalParseSortOrderError<E> {
    InvalidPrefix,
    InvalidKey(E),
}

impl<E> std::error::Error for InternalParseSortOrderError<E>
where
    E: std::error::Error + 'static,
{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            InternalParseSortOrderError::InvalidPrefix => None,
            InternalParseSortOrderError::InvalidKey(err) => Some(err),
        }
    }
}

impl<E> Display for InternalParseSortOrderError<E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InternalParseSortOrderError::InvalidPrefix => f.write_str("invalid sort order prefix"),
            InternalParseSortOrderError::InvalidKey(_) => f.write_str("invalid sort key"),
        }
    }
}

impl<T: FromStr> FromStr for SortOrder<T> {
    type Err = ParseSortOrderError<T::Err>;

    fn from_str(s: &str) -> Result<SortOrder<T>, Self::Err> {
        fn invalid_key<E>(e: E) -> ParseSortOrderError<E> {
            ParseSortOrderError {
                _priv: InternalParseSortOrderError::InvalidKey(e),
            }
        }

        if let Some(key) = s.strip_prefix('+') {
            T::from_str(key)
                .map_err(invalid_key)
                .map(SortOrder::Ascending)
        } else if let Some(key) = s.strip_prefix('-') {
            T::from_str(key)
                .map_err(invalid_key)
                .map(SortOrder::Descending)
        } else {
            Err(ParseSortOrderError {
                _priv: InternalParseSortOrderError::InvalidPrefix,
            })
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
        String::deserialize(deserializer)?
            .parse()
            .map_err(serde::de::Error::custom)
    }
}
