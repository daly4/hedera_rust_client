use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::convert::TryFrom;

use crate::assessed_custom_fee::AssessedCustomFee;
use crate::contract_function_result::ContractFunctionResult;
use crate::error::HederaError;
use crate::proto::services::{self};
use crate::token_transfer_list::{
    proto_token_transfer_list_vec_to_token_transfer_list_hashmap,
    proto_transfer_list_to_account_token_transfer_hashmap, AccountIdTokenTransferHashMap,
    TokenIdTokenTransferListHashMap,
};
use crate::transaction_id::TransactionId;
use crate::transaction_receipt::TransactionReceipt;
use crate::transfer::Transfer;
use crate::utils;
use crate::AccountId;
use crate::Entropy;
use crate::Hbar;
use crate::PublicKey;
use crate::TokenAssociation;

#[derive(Debug, Clone, PartialEq)]
pub struct TransactionRecord {
    pub receipt: TransactionReceipt,
    pub transaction_hash: Vec<u8>,
    pub consensus_timestamp: Option<DateTime<Utc>>,
    pub transaction_id: TransactionId,
    pub transaction_memo: String,
    pub transaction_fee: Hbar,
    pub transfers: AccountIdTokenTransferHashMap,
    pub token_transfers: TokenIdTokenTransferListHashMap,
    pub call_result: Option<ContractFunctionResult>,
    pub call_result_is_create: bool,
    pub assessed_custom_fees: Vec<AssessedCustomFee>,
    pub automatic_token_associations: Vec<TokenAssociation>,
    pub parent_consensus_timestamp: Option<DateTime<Utc>>,
    pub alias_key: Option<PublicKey>,
    pub ethereum_hash: Vec<u8>,
    pub paid_staking_rewards: HashMap<AccountId, Hbar>,
    pub entropy: Option<Entropy>,
}

impl TryFrom<services::TransactionRecord> for TransactionRecord {
    type Error = HederaError;
    fn try_from(services: services::TransactionRecord) -> Result<TransactionRecord, Self::Error> {
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
            transfers: services.transfer_list.map_or_else(
                || Ok(AccountIdTokenTransferHashMap::new()),
                |x| proto_transfer_list_to_account_token_transfer_hashmap(x),
            )?,
            token_transfers: proto_token_transfer_list_vec_to_token_transfer_list_hashmap(
                services.token_transfer_lists,
            )?,
            call_result,
            call_result_is_create,
            assessed_custom_fees: services
                .assessed_custom_fees
                .into_iter()
                .map(AssessedCustomFee::try_from)
                .collect::<Result<Vec<AssessedCustomFee>, HederaError>>()?,
            automatic_token_associations: services
                .automatic_token_associations
                .into_iter()
                .map(TokenAssociation::try_from)
                .collect::<Result<Vec<TokenAssociation>, HederaError>>()?,
            parent_consensus_timestamp: utils::optional_timestamp(
                services.parent_consensus_timestamp,
            )?,
            alias_key: PublicKey::from_bytes(services.alias).ok(),
            ethereum_hash: services.ethereum_hash,
            paid_staking_rewards: services
                .paid_staking_rewards
                .into_iter()
                .map(Transfer::try_from)
                .collect::<Result<Vec<Transfer>, HederaError>>()?
                .into_iter()
                .map(|x| (x.account_id, x.amount))
                .collect::<HashMap<AccountId, Hbar>>(),
            entropy: services.entropy.map(Entropy::from),
        })
    }
}
