use chrono::{DateTime, Utc};
use std::convert::TryFrom;

use crate::error::HederaError;
use crate::key::Key;
use crate::key_list::KeyList;
use crate::proto::services;
use crate::utils;
use crate::AccountId;
use crate::ScheduleId;
use crate::TransactionId;

#[derive(Debug, Clone)]
pub struct ScheduleInfo {
    pub schedule_id: ScheduleId,
    pub creator_account_id: Option<AccountId>,
    pub payer_account_id: Option<AccountId>,
    pub transaction_body: Option<services::SchedulableTransactionBody>,
    pub signatories: Option<KeyList>,
    pub admin_key: Option<Key>,
    pub scheduled_transaction_id: Option<TransactionId>,
    pub memo: String,
    pub expiration_time: Option<DateTime<Utc>>,
    pub executed_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl TryFrom<services::ScheduleInfo> for ScheduleInfo {
    type Error = HederaError;
    fn try_from(services: services::ScheduleInfo) -> Result<ScheduleInfo, Self::Error> {
        let (executed_at, deleted_at) = match services.data {
            Some(d) => match d {
                services::schedule_info::Data::DeletionTime(time) => {
                    let ts = DateTime::<Utc>::try_from(time)?;
                    (None, Some(ts))
                }
                services::schedule_info::Data::ExecutionTime(time) => {
                    let ts = DateTime::<Utc>::try_from(time)?;
                    (Some(ts), None)
                }
            },
            None => (None, None),
        };
        Ok(ScheduleInfo {
            schedule_id: utils::non_optional_schedule_id(services.schedule_id)?,
            creator_account_id: services
                .creator_account_id
                .and_then(|v| AccountId::try_from(v).ok()),
            payer_account_id: services
                .payer_account_id
                .and_then(|v| AccountId::try_from(v).ok()),
            transaction_body: services.scheduled_transaction_body,
            signatories: utils::optional_key_list(services.signers)?,
            admin_key: utils::optional_key(services.admin_key)?,
            scheduled_transaction_id: utils::optional_transaction_id(services.scheduled_transaction_id)?,
            memo: services.memo,
            expiration_time: utils::optional_timestamp(services.expiration_time)?,
            executed_at,
            deleted_at,
        })
    }
}
