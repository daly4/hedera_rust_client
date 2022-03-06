use std::convert::TryFrom;

use crate::error::HederaError;
use crate::hbar::Hbar;
use crate::proto::services::FixedFee as ProtoFixedFee;
use crate::proto::ToProto;
use crate::token_id::TokenId;

#[derive(Debug, Clone, PartialEq)]
pub struct CustomFixedFee {
    pub amount: Hbar,
    pub denominating_token_id: Option<TokenId>,
}

impl TryFrom<ProtoFixedFee> for CustomFixedFee {
    type Error = HederaError;
    fn try_from(services: ProtoFixedFee) -> Result<CustomFixedFee, Self::Error> {
        Ok(CustomFixedFee {
            amount: Hbar::from_tinybar(services.amount),
            denominating_token_id: services.denominating_token_id.map(|x| x.into()),
        })
    }
}

impl ToProto<ProtoFixedFee> for CustomFixedFee {
    fn to_proto(&self) -> std::result::Result<ProtoFixedFee, HederaError> {
        let denominating_token_id = match &self.denominating_token_id {
            Some(x) => Some(x.to_proto()?),
            None => None,
        };
        Ok(ProtoFixedFee {
            amount: self.amount.as_tinybar(),
            denominating_token_id,
        })
    }
}
