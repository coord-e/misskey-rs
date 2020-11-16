use std::fmt::Display;
use std::str::FromStr;

use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

pub struct WithString<T>(pub T);

impl<T> WithString<T> {
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<'de, T> Deserialize<'de> for WithString<T>
where
    T: FromStr,
    T::Err: Display,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map(WithString)
            .map_err(de::Error::custom)
    }
}

impl<T> Serialize for WithString<T>
where
    T: Display,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: Display,
        S: Serializer,
    {
        serializer.collect_str(&self.0)
    }
}

pub mod string {
    use std::fmt::Display;
    use std::str::FromStr;

    use super::WithString;

    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: Display,
        S: Serializer,
    {
        WithString(value).serialize(serializer)
    }

    pub fn deserialize<'de, T, D>(deserializer: D) -> Result<T, D::Error>
    where
        T: FromStr,
        T::Err: Display,
        D: Deserializer<'de>,
    {
        Ok(WithString::<T>::deserialize(deserializer)?.into_inner())
    }
}

pub fn serialize_string_option<T, S>(opt: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
where
    T: Display,
    S: Serializer,
{
    opt.as_ref().map(WithString).serialize(serializer)
}

pub fn serialize_string_vec_option<T, S>(
    opt: &Option<Vec<T>>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    T: Display,
    S: Serializer,
{
    opt.as_ref()
        .map(|vec| vec.iter().map(WithString).collect::<Vec<_>>())
        .serialize(serializer)
}
