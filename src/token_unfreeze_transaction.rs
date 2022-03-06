use crate::entity_id::validate_option_id_checksum;

use crate::transaction::Transaction;
use crate::AccountId;
use crate::Client;
use crate::Hbar;
use crate::HederaError;
use crate::TokenId;
use hedera_derive::{TransactionExecute, TransactionProto, TransactionSchedule};

#[derive(TransactionSchedule, TransactionExecute, Debug, Clone)]
#[hedera_derive(service(
    method_service_name = "token",
    method_service_fn = "unfreeze_token_account"
))]
pub struct TokenUnfreezeTransaction {
    transaction: Transaction,
    services: Proto,
}

impl TokenUnfreezeTransaction {
    pub fn new() -> TokenUnfreezeTransaction {
        let transaction = Transaction::with_max_transaction_fee(Hbar::new(30.0));
        let services = Proto::new();
        TokenUnfreezeTransaction { transaction, services }
    }

    fn validate_network_on_ids(&self, client: &Client) -> Result<(), HederaError> {
        validate_option_id_checksum(&self.services.token, client)?;
        validate_option_id_checksum(&self.services.account, client)?;
        Ok(())
    }

    // token
    gen_transaction_token_fns!();

    // account_id
    gen_transaction_account_fns!();
}

#[derive(Debug, Clone, TransactionProto)]
#[hedera_derive(proto(
    proto_enum = "TokenUnfreeze",
    proto_type = "TokenUnfreezeAccountTransactionBody"
))]
struct Proto {
    #[hedera_derive(to_option_proto)]
    pub token: Option<TokenId>,
    #[hedera_derive(to_option_proto)]
    pub account: Option<AccountId>,
}

impl Proto {
    pub fn new() -> Self {
        Proto {
            token: None,
            account: None,
        }
    }
}
