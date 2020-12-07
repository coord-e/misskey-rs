use std::fmt::{self, Display};
use std::str::FromStr;

use chrono::{DateTime, TimeZone, Utc};
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ObjectId {
    timestamp: u32,
    random: u64,
}

impl ObjectId {
    pub fn datetime(&self) -> DateTime<Utc> {
        Utc.timestamp_millis(self.timestamp as i64 * 1000)
    }
}

#[derive(Debug, Error, Clone)]
#[error("invalid object id")]
pub struct ParseObjectIdError {
    _priv: (),
}

impl FromStr for ObjectId {
    type Err = ParseObjectIdError;

    fn from_str(s: &str) -> Result<ObjectId, Self::Err> {
        let (timestamp_str, random_str) = s.split_at(s.len() - 16);

        let timestamp = match u32::from_str_radix(timestamp_str, 16) {
            Ok(x) => x,
            Err(_) => return Err(ParseObjectIdError { _priv: () }),
        };
        let random = match u64::from_str_radix(random_str, 16) {
            Ok(x) => x,
            Err(_) => return Err(ParseObjectIdError { _priv: () }),
        };

        Ok(ObjectId { timestamp, random })
    }
}

impl Display for ObjectId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:08x}{:016x}", self.timestamp, self.random)
    }
}

impl Serialize for ObjectId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(self)
    }
}

impl<'de> Deserialize<'de> for ObjectId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::ObjectId;
    use chrono::{DateTime, Duration, TimeZone, Utc};
    use rand::{self, Rng};

    fn new() -> ObjectId {
        from_datetime(Utc::now())
    }

    fn from_datetime<Tz>(datetime: DateTime<Tz>) -> ObjectId
    where
        Tz: TimeZone,
    {
        from_datetime_with_source(datetime, &mut rand::thread_rng())
    }

    fn from_datetime_with_source<Tz, R>(datetime: DateTime<Tz>, source: &mut R) -> ObjectId
    where
        Tz: TimeZone,
        R: Rng,
    {
        let timestamp = (datetime.timestamp_millis() as f64 / 1000.0).floor() as u32;
        let random = source.gen::<u64>();
        ObjectId { timestamp, random }
    }

    #[test]
    fn test_deserialize_const() {
        let string1 = "5f8b0eb37844631f2660354b";
        let meid: ObjectId = string1.parse().expect("failed to parse");
        assert_eq!(meid.datetime(), Utc.timestamp_millis(1602948787000));
    }

    #[test]
    fn test_serialize_deserialize() {
        let meid1 = new();
        let string = meid1.to_string();
        let meid2: ObjectId = string.parse().expect("failed to parse");
        assert_eq!(meid1, meid2);
    }

    #[test]
    fn test_deserialize_serialize() {
        let string1 = "5f8b0eb37844631f2660354b";
        let meid: ObjectId = string1.parse().expect("failed to parse");
        let string2 = meid.to_string();
        assert_eq!(string1, string2);
    }

    #[test]
    fn test_order() {
        let time = Utc::now();
        let meid1 = from_datetime(time);
        let meid2 = from_datetime(time + Duration::seconds(1));
        assert!(meid1 < meid2);
    }
}
