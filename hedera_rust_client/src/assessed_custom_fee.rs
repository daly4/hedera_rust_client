use crate::error::HederaError;
use crate::proto::services::{self};
use crate::AccountId;
use crate::TokenId;
use std::convert::TryFrom;

#[derive(Clone, Debug, PartialEq)]
pub struct AssessedCustomFee {
    pub amount: i64,
    pub token_id: Option<TokenId>,
    pub fee_collector_account_id: Option<AccountId>,
    pub effective_payer_account_id: Vec<AccountId>,
}

impl TryFrom<services::AssessedCustomFee> for AssessedCustomFee {
    type Error = HederaError;
    fn try_from(services: services::AssessedCustomFee) -> Result<AssessedCustomFee, Self::Error> {
        Ok(AssessedCustomFee {
            amount: services.amount,
            token_id: services.token_id.map(|x| x.into()),
            fee_collector_account_id: services
                .fee_collector_account_id
                .map(AccountId::try_from)
                .transpose()?,
            effective_payer_account_id: services
                .effective_payer_account_id
                .into_iter()
                .map(AccountId::try_from)
                .collect::<Result<Vec<AccountId>, HederaError>>()?,
        })
    }
}
