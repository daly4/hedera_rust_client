use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use itertools::Itertools;
use std::convert::{TryFrom, TryInto};
use std::str::FromStr;

use crate::error::HederaError;
use crate::proto::{services, ToProto};

#[derive(Debug, Clone, PartialEq)]
pub struct HederaTimestamp(pub i64, pub i32);

impl TryFrom<HederaTimestamp> for DateTime<Utc> {
    type Error = crate::error::HederaError;

    fn try_from(
        HederaTimestamp(seconds, nanos): HederaTimestamp,
    ) -> Result<DateTime<Utc>, HederaError> {
        Ok(Utc.from_utc_datetime(&NaiveDateTime::from_timestamp(
            seconds,
            u32::try_from(nanos)?,
        )))
    }
}

impl TryFrom<DateTime<Utc>> for HederaTimestamp {
    type Error = HederaError;
    fn try_from(dt: DateTime<Utc>) -> Result<HederaTimestamp, Self::Error> {
        Ok(HederaTimestamp(
            dt.timestamp(),
            i32::try_from(dt.timestamp_subsec_nanos())?,
        ))
    }
}

impl TryFrom<services::TimestampSeconds> for DateTime<Utc> {
    type Error = HederaError;
    fn try_from(dt: services::TimestampSeconds) -> Result<DateTime<Utc>, Self::Error> {
        let ts = HederaTimestamp(dt.seconds, 0).try_into()?;
        Ok(ts)
    }
}

impl TryFrom<services::Timestamp> for DateTime<Utc> {
    type Error = HederaError;
    fn try_from(dt: services::Timestamp) -> Result<DateTime<Utc>, Self::Error> {
        let ts = HederaTimestamp(dt.seconds, dt.nanos).try_into()?;
        Ok(ts)
    }
}

impl ToProto<services::Timestamp> for DateTime<Utc> {
    fn to_proto(&self) -> Result<services::Timestamp, HederaError> {
        Ok(services::Timestamp {
            seconds: self.timestamp(),
            nanos: i32::try_from(self.timestamp_subsec_nanos())?,
        })
    }
}

impl ToProto<services::TimestampSeconds> for DateTime<Utc> {
    fn to_proto(&self) -> Result<services::TimestampSeconds, HederaError> {
        Ok(services::TimestampSeconds {
            seconds: self.timestamp(),
        })
    }
}

impl FromStr for HederaTimestamp {
    type Err = HederaError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (seconds, nanos) = s
            .split('.')
            .next_tuple()
            .ok_or_else(|| HederaError::UnknownTimestampFormat)?;
        Ok(HederaTimestamp(seconds.parse()?, nanos.parse()?))
    }
}
