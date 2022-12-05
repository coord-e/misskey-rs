use std::fmt::{self, Display};
use std::str::FromStr;

use chrono::{DateTime, TimeZone, Utc};
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Meid {
    pub timestamp: i64,
    /// 0 - 2^48
    pub random: u64,
}

impl Meid {
    pub fn datetime(&self) -> DateTime<Utc> {
        // NOTE: this does not panic when parsed from valid Meid since the following does not panic
        // `Utc.timestamp_millis_opt(16_i64.pow(12) - 1).unwrap()`
        Utc.timestamp_millis_opt(self.timestamp).unwrap()
    }
}

// https://github.com/misskey-dev/misskey/blob/12.75.1/src/misc/id/meid.ts#L9
const TIME_OFFSET: i64 = 0x800000000000;

#[derive(Debug, Error, Clone)]
#[error("invalid meid")]
pub struct ParseMeidError {
    _priv: (),
}

impl FromStr for Meid {
    type Err = ParseMeidError;

    fn from_str(s: &str) -> Result<Meid, Self::Err> {
        let (timestamp_str, random_str) = s.split_at(s.len() - 12);

        let timestamp = match i64::from_str_radix(timestamp_str, 16) {
            Ok(0) => 0,
            Ok(x) => x - TIME_OFFSET,
            Err(_) => return Err(ParseMeidError { _priv: () }),
        };

        let random = match u64::from_str_radix(random_str, 16) {
            Ok(x) => x,
            Err(_) => return Err(ParseMeidError { _priv: () }),
        };

        Ok(Meid { timestamp, random })
    }
}

impl Display for Meid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let with_offset = if self.timestamp <= 0 {
            0
        } else {
            self.timestamp + TIME_OFFSET
        };
        write!(
            f,
            "{:012x}{:012x}",
            with_offset,
            self.random % 2_u64.pow(48)
        )
    }
}

impl Serialize for Meid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(self)
    }
}

impl<'de> Deserialize<'de> for Meid {
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
    use super::Meid;
    use chrono::{DateTime, Duration, TimeZone, Utc};
    use rand::{self, Rng};

    fn new() -> Meid {
        from_datetime(Utc::now())
    }

    fn from_datetime<Tz>(datetime: DateTime<Tz>) -> Meid
    where
        Tz: TimeZone,
    {
        from_datetime_with_source(datetime, &mut rand::thread_rng())
    }

    fn from_datetime_with_source<Tz, R>(datetime: DateTime<Tz>, source: &mut R) -> Meid
    where
        Tz: TimeZone,
        R: Rng,
    {
        let timestamp = datetime.timestamp_millis();
        let random = source.gen::<u64>() % 2_u64.pow(48);
        Meid { timestamp, random }
    }

    #[test]
    fn test_deserialize_const() {
        let string = "817537316bb2ef661de6af11";
        let meid: Meid = string.parse().expect("failed to parse");
        assert_eq!(meid.datetime(), Utc.timestamp_millis(1602948787122));
    }

    #[test]
    fn test_serialize_deserialize() {
        let meid1 = new();
        let string = meid1.to_string();
        let meid2: Meid = string.parse().expect("failed to parse");
        assert_eq!(meid1, meid2);
    }

    #[test]
    fn test_deserialize_serialize() {
        let string1 = "817537316bb2ef661de6af11";
        let meid: Meid = string1.parse().expect("failed to parse");
        let string2 = meid.to_string();
        assert_eq!(string1, string2);
    }

    #[test]
    fn test_order() {
        let time = Utc::now();
        let meid1 = from_datetime(time);
        let meid2 = from_datetime(time + Duration::milliseconds(1));
        assert!(meid1 < meid2);
    }
}
