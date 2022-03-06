use std::convert::TryFrom;

use crate::error::HederaError;
use crate::proto::{services, ToProto};
use crate::AccountId;
use crate::Hbar;

#[derive(Debug, Clone)]
pub struct Transfer {
    pub account_id: AccountId,
    pub amount: Hbar,
}

impl TryFrom<services::ProxyStaker> for Transfer {
    type Error = HederaError;
    fn try_from(services: services::ProxyStaker) -> Result<Transfer, Self::Error> {
        Ok(Transfer {
            account_id: match services.account_id {
                Some(account_id) => AccountId::try_from(account_id)?,
                None => return Err(HederaError::MissingInProto("account_id".to_string())),
            },
            amount: Hbar::from_tinybar(services.amount),
        })
    }
}

impl TryFrom<services::AccountAmount> for Transfer {
    type Error = HederaError;
    fn try_from(services: services::AccountAmount) -> Result<Transfer, Self::Error> {
        Ok(Transfer {
            account_id: match services.account_id {
                Some(account_id) => AccountId::try_from(account_id)?,
                None => return Err(HederaError::MissingInProto("account_id".to_string())),
            },
            amount: Hbar::from_tinybar(services.amount),
        })
    }
}

impl ToProto<services::AccountAmount> for Transfer {
    fn to_proto(&self) -> Result<services::AccountAmount, HederaError> {
        Ok(services::AccountAmount {
            account_id: Some(self.account_id.to_proto()?),
            amount: self.amount.as_tinybar(),
            is_approval: false,
        })
    }
}
