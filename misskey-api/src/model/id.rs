use std::cmp::Ordering;
use std::fmt::{self, Debug, Display};
use std::hash::{self, Hash};
use std::marker::PhantomData;
use std::str::FromStr;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use thiserror::Error;

pub mod aid;
pub mod meid;
pub mod object_id;

#[cfg(feature = "aid")]
type IdImpl = aid::Aid;
#[cfg(feature = "meid")]
type IdImpl = meid::Meid;
#[cfg(feature = "ulid")]
type IdImpl = ulid_crate::Ulid;
#[cfg(feature = "objectid")]
type IdImpl = object_id::ObjectId;

pub struct Id<T: ?Sized> {
    inner: IdImpl,
    _marker: PhantomData<fn() -> T>,
}

impl<T: ?Sized> Id<T> {
    #[cfg(any(docsrs, feature = "aid"))]
    #[cfg_attr(docsrs, doc(cfg(feature = "aid")))]
    pub fn as_aid(&self) -> &aid::Aid {
        &self.inner
    }

    #[cfg(any(docsrs, feature = "meid"))]
    #[cfg_attr(docsrs, doc(cfg(feature = "meid")))]
    pub fn as_meid(&self) -> &meid::Meid {
        &self.inner
    }

    #[cfg(any(docsrs, feature = "ulid"))]
    #[cfg_attr(docsrs, doc(cfg(feature = "ulid")))]
    pub fn as_ulid(&self) -> &ulid_crate::Ulid {
        &self.inner
    }

    #[cfg(any(docsrs, feature = "objectid"))]
    #[cfg_attr(docsrs, doc(cfg(feature = "objectid")))]
    pub fn as_object_id(&self) -> &object_id::ObjectId {
        &self.inner
    }
}

#[cfg(feature = "aid")]
impl<T: ?Sized> From<Id<T>> for aid::Aid {
    fn from(id: Id<T>) -> aid::Aid {
        *id.as_aid()
    }
}

#[cfg(feature = "meid")]
impl<T: ?Sized> From<Id<T>> for meid::Meid {
    fn from(id: Id<T>) -> meid::Meid {
        *id.as_meid()
    }
}

#[cfg(feature = "ulid")]
impl<T: ?Sized> From<Id<T>> for ulid_crate::Ulid {
    fn from(id: Id<T>) -> ulid_crate::Ulid {
        *id.as_ulid()
    }
}

#[cfg(feature = "objectid")]
impl<T: ?Sized> From<Id<T>> for object_id::ObjectId {
    fn from(id: Id<T>) -> object_id::ObjectId {
        *id.as_object_id()
    }
}

// `derive` fails to infer correct trait bounds on phantom type parameter,
// so just implementing it manually

impl<T> Debug for Id<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(&self.inner, f)
    }
}

impl<T> PartialOrd for Id<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.inner.partial_cmp(&other.inner)
    }
}

impl<T> Ord for Id<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.inner.cmp(&other.inner)
    }
}

impl<T> PartialEq for Id<T> {
    fn eq(&self, other: &Self) -> bool {
        self.inner.eq(&other.inner)
    }
}

impl<T> Eq for Id<T> {}

impl<T> Clone for Id<T> {
    fn clone(&self) -> Self {
        Id {
            inner: self.inner,
            _marker: PhantomData,
        }
    }
}

impl<T> Copy for Id<T> {}

impl<T> Hash for Id<T> {
    fn hash<H>(&self, state: &mut H)
    where
        H: hash::Hasher,
    {
        self.inner.hash(state);
    }
}

impl<T> Id<T> {
    pub fn datetime(&self) -> DateTime<Utc> {
        self.inner.datetime()
    }
}

#[derive(Debug, Error, Clone)]
#[error("invalid id")]
pub struct ParseIdError {
    _priv: (),
}

impl<T> FromStr for Id<T> {
    type Err = ParseIdError;

    fn from_str(s: &str) -> Result<Id<T>, Self::Err> {
        IdImpl::from_str(s)
            .map(|inner| Id {
                inner,
                _marker: PhantomData,
            })
            .map_err(|_| ParseIdError { _priv: () })
    }
}

impl<T> Display for Id<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.inner, f)
    }
}

impl<T> Serialize for Id<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.inner.serialize(serializer)
    }
}

impl<'de, T> Deserialize<'de> for Id<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        IdImpl::deserialize(deserializer).map(|inner| Id {
            inner,
            _marker: PhantomData,
        })
    }
}
