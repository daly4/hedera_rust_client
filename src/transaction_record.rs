use chrono::{DateTime, Utc};
use std::convert::TryFrom;

use crate::contract_function_result::ContractFunctionResult;
use crate::error::HederaError;
use crate::proto::services::{self};
use crate::transaction_id::TransactionId;
use crate::transaction_receipt::TransactionReceipt;
use crate::transfer::Transfer;
use crate::utils;
use crate::Hbar;

#[derive(Debug, Clone)]
pub struct TransactionRecord {
    pub receipt: TransactionReceipt,
    pub transaction_hash: Vec<u8>,
    pub consensus_timestamp: Option<DateTime<Utc>>,
    pub transaction_id: TransactionId,
    pub transaction_memo: String,
    pub transaction_fee: Hbar,
    pub transfers: Vec<Transfer>,
    pub call_result: Option<ContractFunctionResult>,
    pub call_result_is_create: bool,
}

impl TryFrom<services::TransactionRecord> for TransactionRecord {
    type Error = HederaError;
    fn try_from(services: services::TransactionRecord) -> Result<TransactionRecord, Self::Error> {
        let transfers = match services.transfer_list {
            Some(list) => {
                let amounts = list
                    .account_amounts
                    .into_iter()
                    .map(|v| Transfer::try_from(v))
                    .collect::<Result<Vec<Transfer>, HederaError>>()?;
                amounts
            }
            None => Vec::new(),
        };

        let (call_result_is_create, call_result) = match services.body {
            Some(body) => match body {
                services::transaction_record::Body::ContractCallResult(call_result) => {
                    (false, Some(ContractFunctionResult::try_from(call_result)?))
                }
                services::transaction_record::Body::ContractCreateResult(call_result) => {
                    (true, Some(ContractFunctionResult::try_from(call_result)?))
                }
            },
            None => (false, None),
        };

        Ok(TransactionRecord {
            receipt: match services.receipt {
                Some(v) => TransactionReceipt::try_from(v)?,
                None => return Err(HederaError::MissingInProto("receipt".to_string())),
            },
            transaction_hash: services.transaction_hash,
            consensus_timestamp: utils::optional_timestamp(services.consensus_timestamp)?,
            transaction_id: utils::non_optional_transaction_id(services.transaction_id)?,
            transaction_memo: services.memo,
            transaction_fee: Hbar::try_from(services.transaction_fee)?,
            transfers,
            call_result,
            call_result_is_create,
        })
    }
}
