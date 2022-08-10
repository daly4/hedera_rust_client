use chrono::{DateTime, Utc};
use std::convert::{TryFrom, TryInto};

use crate::error::HederaError;
use crate::proto::services::{
    FeeSchedule as ProtoFeeSchedule, TransactionFeeSchedule as ProtoTransactionFeeSchedule,
};
use crate::proto::ToProto;
use crate::transaction_fee_schedule::TransactionFeeSchedule;

#[derive(Debug, Clone, PartialEq)]
pub struct FeeSchedule {
    pub transaction_fee_schedule: Vec<TransactionFeeSchedule>,
    pub expiry_time: Option<DateTime<Utc>>,
}

impl TryFrom<ProtoFeeSchedule> for FeeSchedule {
    type Error = HederaError;
    fn try_from(services: ProtoFeeSchedule) -> Result<FeeSchedule, Self::Error> {
        let expiry_time = match services.expiry_time {
            Some(x) => Some(x.try_into()?),
            None => None,
        };
        let transaction_fee_schedule = services
            .transaction_fee_schedule
            .into_iter()
            .map(|x| x.try_into())
            .collect::<Result<Vec<TransactionFeeSchedule>, HederaError>>()?;
        Ok(FeeSchedule {
            transaction_fee_schedule,
            expiry_time,
        })
    }
}

impl ToProto<ProtoFeeSchedule> for FeeSchedule {
    fn to_proto(&self) -> Result<ProtoFeeSchedule, HederaError> {
        let expiry_time = match self.expiry_time {
            Some(x) => Some(x.to_proto()?),
            None => None,
        };
        let transaction_fee_schedule =
            self.transaction_fee_schedule
                .iter()
                .map(|x| x.to_proto())
                .collect::<Result<Vec<ProtoTransactionFeeSchedule>, HederaError>>()?;
        Ok(ProtoFeeSchedule {
            transaction_fee_schedule,
            expiry_time,
        })
    }
}
