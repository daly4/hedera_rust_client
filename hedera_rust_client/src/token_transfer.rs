use std::convert::TryFrom;

use crate::error::HederaError;
use crate::proto::services::AccountAmount as ProtoAccountAmount;
use crate::proto::ToProto;
use crate::AccountId;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TokenTransfer {
    pub account_id: AccountId,
    pub amount: i64,
}

impl TryFrom<ProtoAccountAmount> for TokenTransfer {
    type Error = HederaError;
    fn try_from(services: ProtoAccountAmount) -> Result<TokenTransfer, Self::Error> {
        Ok(TokenTransfer {
            account_id: match services.account_id {
                Some(account_id) => AccountId::try_from(account_id)?,
                None => return Err(HederaError::MissingInProto("account_id".to_string())),
            },
            amount: services.amount,
        })
    }
}

impl ToProto<ProtoAccountAmount> for TokenTransfer {
    fn to_proto(&self) -> Result<ProtoAccountAmount, HederaError> {
        Ok(ProtoAccountAmount {
            account_id: Some(self.account_id.to_proto()?),
            amount: self.amount,
            is_approval: false,
        })
    }
}
