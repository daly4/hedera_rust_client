use chrono::{DateTime, Duration, Utc};
use std::convert::TryFrom;

use crate::error::HederaError;
use crate::key::Key;
use crate::proto::services;
use crate::utils;
use crate::AccountId;
use crate::TokenFreezeStatus;
use crate::TokenId;
use crate::TokenKycStatus;

#[derive(Debug, Clone, PartialEq)]
pub struct TokenInfo {
    /// ID of the token instance
    pub token_id: TokenId,
    /// The name of the token. It is a string of ASCII only characters
    pub name: String,
    /// The symbol of the token. It is a UTF-8 capitalized alphabetical string
    pub symbol: String,
    /// The number of decimal places a token is divisible by
    pub decimals: u32,
    /// The total supply of tokens that are currently in circulation
    pub total_supply: u64,
    /// The ID of the account which is set as Treasury
    pub treasury: AccountId,
    /// The key which can perform update/delete operations on the token. If empty, the token can be perceived as immutable (not being able to be updated/deleted)
    pub admin_key: Option<Key>,
    /// The key which can grant or revoke KYC of an account for the token's transactions. If empty, KYC is not required, and KYC grant or revoke operations are not possible.
    pub kyc_key: Option<Key>,
    /// The key which can freeze or unfreeze an account for token transactions. If empty, freezing is not possible
    pub freeze_key: Option<Key>,
    /// The key which can wipe token balance of an account. If empty, wipe is not possible
    pub wipe_key: Option<Key>,
    /// The key which can change the supply of a token. The key is used to sign Token Mint/Burn operations
    pub supply_key: Option<Key>,
    /// The default Freeze status (not applicable, frozen or unfrozen) of Hedera accounts relative to this token. FreezeNotApplicable is returned if Token Freeze Key is empty. Frozen is returned if Token Freeze Key is set and defaultFreeze is set to true. Unfrozen is returned if Token Freeze Key is set and defaultFreeze is set to false
    pub default_freeze_status: TokenFreezeStatus,
    /// The default KYC status (KycNotApplicable or Revoked) of Hedera accounts relative to this token. KycNotApplicable is returned if KYC key is not set, otherwise Revoked
    pub default_kyc_status: TokenKycStatus,
    /// Specifies whether the token was deleted or not
    pub deleted: bool,
    /// An account which will be automatically charged to renew the token's expiration, at autoRenewPeriod interval
    pub auto_renew_account: Option<AccountId>,
    /// The interval at which the auto-renew account will be charged to extend the token's expiry
    pub auto_renew_period: Option<Duration>,
    /// The epoch second at which the token will expire
    pub expiry: Option<DateTime<Utc>>,
    /// The memo associated with the token
    pub token_memo: String,
}

impl TryFrom<services::TokenInfo> for TokenInfo {
    type Error = HederaError;
    fn try_from(services: services::TokenInfo) -> Result<TokenInfo, Self::Error> {
        let default_kyc_status = TokenKycStatus::from_i32(services.default_kyc_status)
            .ok_or(HederaError::UnexpectedProtoType)?;
        let default_freeze_status = TokenFreezeStatus::from_i32(services.default_kyc_status)
            .ok_or(HederaError::UnexpectedProtoType)?;
        Ok(TokenInfo {
            token_id: utils::non_optional_token_id(services.token_id)?,
            name: services.name,
            symbol: services.symbol,
            decimals: services.decimals,
            total_supply: services.total_supply,
            treasury: utils::non_optional_account_id(services.treasury)?,
            admin_key: utils::optional_key(services.admin_key)?,
            kyc_key: utils::optional_key(services.kyc_key)?,
            freeze_key: utils::optional_key(services.freeze_key)?,
            wipe_key: utils::optional_key(services.wipe_key)?,
            supply_key: utils::optional_key(services.supply_key)?,
            default_freeze_status,
            default_kyc_status,
            deleted: services.deleted,
            auto_renew_account: utils::optional_account_id(services.auto_renew_account)?,
            auto_renew_period: utils::optional_duration(services.auto_renew_period)?,
            expiry: utils::optional_timestamp(services.expiry)?,
            token_memo: services.memo,
        })
    }
}
