use crate::error::HederaError;
use crate::proto::services;
use crate::Hbar;
use crate::TokenId;
use std::{collections::HashMap, convert::TryFrom};

#[derive(Debug, Clone)]
pub struct AccountBalance {
    pub hbars: Hbar,
    pub token: HashMap<TokenId, u64>,
}

impl AccountBalance {
    pub fn new() -> AccountBalance {
        AccountBalance {
            hbars: Hbar::zero(),
            token: HashMap::new(),
        }
    }
}

impl TryFrom<services::CryptoGetAccountBalanceResponse> for AccountBalance {
    type Error = HederaError;

    fn try_from(services: services::CryptoGetAccountBalanceResponse) -> Result<AccountBalance, Self::Error> {
        let mut token = HashMap::new();
        for token_balance in services.token_balances.iter() {
            if let Some(token_id) = &token_balance.token_id {
                token.insert(TokenId::from(token_id.clone()), token_balance.balance);
            }
        }
        let hbars = Hbar::from_tinybar(i64::try_from(services.balance)?);

        Ok(AccountBalance { hbars, token })
    }
}
