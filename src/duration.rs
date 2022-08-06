use chrono::Duration;
use std::convert::TryFrom;

use crate::error::HederaError;
use crate::proto::{services, ToProto};

impl ToProto<services::Duration> for Duration {
    fn to_proto(&self) -> Result<services::Duration, HederaError> {
        Ok(services::Duration {
            seconds: self.num_seconds(),
        })
    }
}

impl TryFrom<services::Duration> for Duration {
    type Error = HederaError;

    fn try_from(duration: services::Duration) -> Result<Duration, Self::Error> {
        Ok(Duration::seconds(duration.seconds))
    }
}
