use chrono::{DateTime, Utc, Duration};
use std::convert::TryFrom;

use crate::error::HederaError;
use crate::key::Key;
use crate::proto::services;
use crate::token_relationship::TokenRelationship;
use crate::utils;
use crate::AccountId;
use crate::ContractId;
use crate::Hbar;

#[derive(Debug, Clone)]
pub struct ContractInfo {
    /// ID of the contract instance, in the format used in transactions
    pub contract_id: Option<ContractId>,
    /// ID of the cryptocurrency account owned by the contract instance, in the format used in transactions
    pub account_id: Option<AccountId>,
    /// ID of both the contract instance and the cryptocurrency account owned by the contract instance, in the format used by Solidity
    pub contract_account_id: String,
    /// the state of the instance and its fields can be modified arbitrarily if this key signs a transaction to modify it. If this is null, then such modifications are not possible, and there is no administrator that can override the normal operation of this smart contract instance. Note that if it is created with no admin keys, then there is no administrator to authorize changing the admin keys, so there can never be any admin keys for that instance. */
    pub admin_key: Option<Key>,
    /// the current time at which this contract instance (and its account) is set to expire
    pub expiration_time: Option<DateTime<Utc>>,
    /// the expiration time will extend every this many seconds. If there are insufficient funds, then it extends as long as possible. If the account is empty when it expires, then it is deleted.
    pub auto_renew_period: Option<Duration>,
    /// number of bytes of storage being used by this instance (which affects the cost to extend the expiration time)
    pub storage: i64,
    /// the memo associated with the contract (max 100 bytes)
    pub memo: String,
    /// The current balance, in tinybars
    pub balance: Hbar,
    /// Whether the contract has been deleted
    pub deleted: bool,
    /// The tokens associated to the contract
    pub token_relationships: Vec<TokenRelationship>,
}

impl TryFrom<services::contract_get_info_response::ContractInfo> for ContractInfo {
    type Error = HederaError;
    fn try_from(
        services: services::contract_get_info_response::ContractInfo,
    ) -> Result<ContractInfo, Self::Error> {
        let token_relationships = services
            .token_relationships
            .into_iter()
            .map(|v| TokenRelationship::try_from(v))
            .collect::<Result<Vec<TokenRelationship>, HederaError>>()?;
        Ok(ContractInfo {
            contract_id: services.contract_id.map(|v| ContractId::from(v)),
            account_id: utils::optional_account_id(services.account_id)?,
            contract_account_id: services.contract_account_id,
            admin_key: utils::optional_key(services.admin_key)?,
            expiration_time: utils::optional_timestamp(services.expiration_time)?,
            auto_renew_period: utils::optional_duration(services.auto_renew_period)?,
            storage: services.storage,
            memo: services.memo,
            balance: Hbar::try_from(services.balance)?,
            deleted: services.deleted,
            token_relationships,
        })
    }
}
