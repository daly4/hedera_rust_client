use chrono::{DateTime, Utc};
use std::convert::{TryFrom, TryInto};

use crate::account_id::AccountId;
use crate::error::HederaError;
use crate::ledger_id::LedgerId;
use crate::nft_id::NftId;
use crate::proto::services::TokenNftInfo as ProtoTokenNftInfo;
use crate::proto::ToProto;

#[derive(Debug, Clone, PartialEq)]
pub struct TokenNftInfo {
    pub nft_id: NftId,
    pub account_id: AccountId,
    pub creation_time: DateTime<Utc>,
    pub metadata: Vec<u8>,
    pub ledger_id: LedgerId,
    pub spender_id: Option<AccountId>,
}

impl TryFrom<ProtoTokenNftInfo> for TokenNftInfo {
    type Error = HederaError;
    fn try_from(services: ProtoTokenNftInfo) -> Result<TokenNftInfo, Self::Error> {
        Ok(TokenNftInfo {
            nft_id: services
                .nft_id
                .ok_or(HederaError::MissingInProto("nft_id".to_string()))?
                .try_into()?,
            account_id: services
                .account_id
                .ok_or(HederaError::MissingInProto("account_id".to_string()))?
                .try_into()?,
            creation_time: services
                .creation_time
                .ok_or(HederaError::MissingInProto("creation_time".to_string()))?
                .try_into()?,
            metadata: services.metadata,
            ledger_id: LedgerId::new(services.ledger_id),
            spender_id: services.spender_id.map(|x| x.try_into()).transpose()?,
        })
    }
}

impl ToProto<ProtoTokenNftInfo> for TokenNftInfo {
    fn to_proto(&self) -> Result<ProtoTokenNftInfo, HederaError> {
        Ok(ProtoTokenNftInfo {
            nft_id: Some(self.nft_id.to_proto()?),
            account_id: Some(self.account_id.to_proto()?),
            creation_time: Some(self.creation_time.to_proto()?),
            metadata: self.metadata.clone(),
            ledger_id: self.ledger_id.as_bytes(),
            spender_id: self.spender_id.map(|x| x.to_proto()).transpose()?,
        })
    }
}
