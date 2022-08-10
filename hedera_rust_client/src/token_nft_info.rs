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
    pub nft_id: Option<NftId>,
    pub account_id: Option<AccountId>,
    pub creation_time: Option<DateTime<Utc>>,
    pub metadata: Vec<u8>,
    pub ledger_id: LedgerId,
    pub spender_id: Option<AccountId>,
}

impl TryFrom<ProtoTokenNftInfo> for TokenNftInfo {
    type Error = HederaError;
    fn try_from(services: ProtoTokenNftInfo) -> Result<TokenNftInfo, Self::Error> {
        Ok(TokenNftInfo {
            nft_id: services.nft_id.map(|x| x.try_into()).transpose()?,
            account_id: services.account_id.map(|x| x.try_into()).transpose()?,
            creation_time: services.creation_time.map(|x| x.try_into()).transpose()?,
            metadata: services.metadata,
            ledger_id: LedgerId::new(services.ledger_id),
            spender_id: services.spender_id.map(|x| x.try_into()).transpose()?,
        })
    }
}

impl ToProto<ProtoTokenNftInfo> for TokenNftInfo {
    fn to_proto(&self) -> Result<ProtoTokenNftInfo, HederaError> {
        Ok(ProtoTokenNftInfo {
            nft_id: self.nft_id.map(|x| x.to_proto()).transpose()?,
            account_id: self.account_id.map(|x| x.to_proto()).transpose()?,
            creation_time: self.creation_time.map(|x| x.to_proto()).transpose()?,
            metadata: self.metadata.clone(),
            ledger_id: self.ledger_id.as_bytes(),
            spender_id: self.spender_id.map(|x| x.to_proto()).transpose()?,
        })
    }
}
