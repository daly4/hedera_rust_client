use chrono::{DateTime, Duration, Utc};
use std::convert::TryFrom;

use crate::error::HederaError;
use crate::key::Key;
use crate::proto::services;
use crate::token_relationship::TokenRelationship;
use crate::utils;
use crate::AccountId;
use crate::Hbar;

#[derive(Debug, Clone)]
pub struct AccountInfo {
    pub account_id: AccountId,
    pub contract_account_id: String,
    pub deleted: bool,
    pub proxy_account_id: Option<AccountId>,
    pub proxy_received: Hbar,
    pub key: Option<Key>,
    pub balance: Hbar,
    pub receiver_sig_required: bool,
    pub expiration_time: Option<DateTime<Utc>>,
    pub auto_renew_period: Option<Duration>,
    pub token_relationships: Vec<TokenRelationship>,
    pub memo: String,
}

impl TryFrom<services::crypto_get_info_response::AccountInfo> for AccountInfo {
    type Error = HederaError;
    fn try_from(
        services: services::crypto_get_info_response::AccountInfo,
    ) -> Result<AccountInfo, Self::Error> {
        let token_relationships = services
            .token_relationships
            .into_iter()
            .map(|v| TokenRelationship::try_from(v))
            .collect::<Result<Vec<TokenRelationship>, HederaError>>()?;
        Ok(AccountInfo {
            account_id: utils::non_optional_account_id(services.account_id)?,
            contract_account_id: services.contract_account_id,
            deleted: services.deleted,
            proxy_account_id: utils::optional_account_id(services.proxy_account_id)?,
            proxy_received: Hbar::from(services.proxy_received),
            key: utils::optional_key(services.key)?,
            balance: Hbar::try_from(services.balance)?,
            receiver_sig_required: services.receiver_sig_required,
            expiration_time: utils::optional_timestamp(services.expiration_time)?,
            auto_renew_period: utils::optional_duration(services.auto_renew_period)?,
            // live_hashes: Vec<LiveHash>,
            token_relationships,
            memo: services.memo,
        })
    }
}
