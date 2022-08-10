use std::convert::TryFrom;

use crate::error::HederaError;
use crate::proto::services;
use crate::status::Status;
use crate::utils;
use crate::AccountId;
use crate::ContractId;
use crate::ExchangeRate;
use crate::FileId;
use crate::ScheduleId;
use crate::TokenId;
use crate::TopicId;
use crate::TransactionId;

#[derive(Debug, Clone, PartialEq)]
pub struct TransactionReceipt {
    pub status: Status,
    pub exchange_rate: Option<ExchangeRate>,
    pub topic_id: Option<TopicId>,
    pub file_id: Option<FileId>,
    pub contract_id: Option<ContractId>,
    pub account_id: Option<AccountId>,
    pub token_id: Option<TokenId>,
    pub topic_sequence_num: u64,
    pub topic_running_hash: Vec<u8>,
    pub topic_running_hash_version: u64,
    pub total_supply: u64,
    pub scheduled_id: Option<ScheduleId>,
    pub scheduled_transaction_id: Option<TransactionId>,
    pub serial_numbers: Vec<i64>,
}

impl TryFrom<services::TransactionReceipt> for TransactionReceipt {
    type Error = HederaError;
    fn try_from(services: services::TransactionReceipt) -> Result<TransactionReceipt, Self::Error> {
        let exchange_rate = match services.exchange_rate {
            Some(rate_pair) => match rate_pair.current_rate {
                Some(rate) => Some(ExchangeRate::try_from(rate)?),
                None => None,
            },
            None => None,
        };

        Ok(TransactionReceipt {
            status: match Status::from_i32(services.status) {
                Some(v) => v,
                None => return Err(HederaError::UnknownHederaStatusCode(services.status)),
            },
            exchange_rate,
            topic_id: services.topic_id.map(TopicId::from),
            file_id: services.file_id.map(FileId::from),
            contract_id: services.contract_id.map(ContractId::from),
            account_id: utils::optional_account_id(services.account_id)?,
            token_id: services.token_id.map(TokenId::from),
            topic_sequence_num: services.topic_sequence_number,
            topic_running_hash: services.topic_running_hash,
            topic_running_hash_version: services.topic_running_hash_version,
            total_supply: services.new_total_supply,
            scheduled_id: services.schedule_id.map(ScheduleId::from),
            scheduled_transaction_id: match services.scheduled_transaction_id {
                Some(v) => Some(TransactionId::try_from(v)?),
                None => None,
            },
            serial_numbers: services.serial_numbers,
        })
    }
}
