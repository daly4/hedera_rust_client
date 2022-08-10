use std::convert::TryFrom;

use crate::error::HederaError;
use crate::proto::services::{self};
use crate::AccountId;
use crate::TokenId;

#[derive(Clone, Debug, PartialEq)]
pub struct TokenAssociation {
    pub token_id: TokenId,
    pub account_id: AccountId,
}

impl TryFrom<services::TokenAssociation> for TokenAssociation {
    type Error = HederaError;
    fn try_from(services: services::TokenAssociation) -> Result<TokenAssociation, Self::Error> {
        Ok(TokenAssociation {
            token_id: TokenId::try_from(
                services
                    .token_id
                    .ok_or(HederaError::MissingInProto("token_id".to_string()))?,
            )?,
            account_id: AccountId::try_from(
                services
                    .account_id
                    .ok_or(HederaError::MissingInProto("account_id".to_string()))?,
            )?,
        })
    }
}
