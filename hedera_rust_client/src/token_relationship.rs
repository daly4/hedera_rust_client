use std::convert::TryFrom;

use crate::error::HederaError;
use crate::proto::services;
use crate::TokenFreezeStatus;
use crate::TokenId;
use crate::TokenKycStatus;

#[derive(Debug, Clone, PartialEq)]
pub struct TokenRelationship {
    pub token_id: TokenId,
    pub symbol: String,
    pub balance: u64,
    pub kyc_status: TokenKycStatus,
    pub freeze_status: TokenFreezeStatus,
}

impl TryFrom<services::TokenRelationship> for TokenRelationship {
    type Error = HederaError;
    fn try_from(services: services::TokenRelationship) -> Result<TokenRelationship, Self::Error> {
        let kyc_status = TokenKycStatus::from_i32(services.kyc_status)
            .ok_or(HederaError::UnexpectedProtoType)?;
        let freeze_status = TokenFreezeStatus::from_i32(services.kyc_status)
            .ok_or(HederaError::UnexpectedProtoType)?;
        Ok(TokenRelationship {
            token_id: match services.token_id {
                Some(v) => TokenId::from(v),
                None => return Err(HederaError::MissingInProto("token_id".to_string())),
            },
            symbol: services.symbol,
            balance: services.balance,
            kyc_status,
            freeze_status,
        })
    }
}
