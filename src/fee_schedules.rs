use std::convert::{TryFrom, TryInto};

use crate::error::HederaError;
use crate::fee_schedule::FeeSchedule;
use crate::proto::services::CurrentAndNextFeeSchedule;
use crate::proto::ToProto;

#[derive(Debug, Clone)]
pub struct FeeSchedules {
    pub current: Option<FeeSchedule>,
    pub next: Option<FeeSchedule>,
}

impl TryFrom<CurrentAndNextFeeSchedule> for FeeSchedules {
    type Error = HederaError;
    fn try_from(services: CurrentAndNextFeeSchedule) -> Result<FeeSchedules, Self::Error> {
        let current = match services.current_fee_schedule {
            Some(x) => Some(x.try_into()?),
            None => None,
        };
        let next = match services.next_fee_schedule {
            Some(x) => Some(x.try_into()?),
            None => None,
        };
        Ok(FeeSchedules { current, next })
    }
}

impl ToProto<CurrentAndNextFeeSchedule> for FeeSchedules {
    fn to_proto(&self) -> Result<CurrentAndNextFeeSchedule, HederaError> {
        let current_fee_schedule = match &self.current {
            Some(x) => Some(x.to_proto()?),
            None => None,
        };
        let next_fee_schedule = match &self.next {
            Some(x) => Some(x.to_proto()?),
            None => None,
        };
        Ok(CurrentAndNextFeeSchedule {
            current_fee_schedule,
            next_fee_schedule,
        })
    }
}
