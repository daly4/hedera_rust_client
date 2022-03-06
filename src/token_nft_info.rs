use chrono::{DateTime, Utc};
use std::convert::{TryFrom, TryInto};

use crate::account_id::AccountId;
use crate::error::HederaError;
use crate::ledger_id::LedgerId;
use crate::nft_id::NftId;
use crate::proto::services::TokenNftInfo as ProtoTokenNftInfo;
use crate::proto::ToProto;

#[derive(Debug, Clone)]
pub struct TokenNftInfo {
    pub nft_id: Option<NftId>,
    pub account_id: Option<AccountId>,
    pub creation_time: Option<DateTime<Utc>>,
    pub metadata: Vec<u8>,
    pub ledger_id: LedgerId,
}

impl TryFrom<ProtoTokenNftInfo> for TokenNftInfo {
    type Error = HederaError;
    fn try_from(services: ProtoTokenNftInfo) -> Result<TokenNftInfo, Self::Error> {
        Ok(TokenNftInfo {
            nft_id: match services.nft_id {
                Some(x) => Some(x.try_into()?),
                None => None,
            },
            account_id: match services.account_id {
                Some(x) => Some(x.try_into()?),
                None => None,
            },
            creation_time: match services.creation_time {
                Some(x) => Some(x.try_into()?),
                None => None,
            },
            metadata: services.metadata,
            ledger_id: LedgerId::new(services.ledger_id),
        })
    }
}

impl ToProto<ProtoTokenNftInfo> for TokenNftInfo {
    fn to_proto(&self) -> Result<ProtoTokenNftInfo, HederaError> {
        Ok(ProtoTokenNftInfo {
            nft_id: match &self.nft_id {
                Some(x) => Some(x.to_proto()?),
                None => None,
            },
            account_id: match &self.account_id {
                Some(x) => Some(x.to_proto()?),
                None => None,
            },
            creation_time: match &self.creation_time {
                Some(x) => Some(x.to_proto()?),
                None => None,
            },
            metadata: self.metadata.clone(),
            ledger_id: self.ledger_id.as_bytes(),
        })
    }
}
