use crate::entity_id::validate_option_id_checksum;

use crate::transaction::Transaction;
use crate::AccountId;
use crate::Client;
use crate::Hbar;
use crate::HederaError;
use crate::TokenId;
use hedera_rust_client_derive::{TransactionExecute, TransactionProto, TransactionSchedule};

#[derive(TransactionSchedule, TransactionExecute, Debug, Clone, PartialEq)]
#[hedera_rust_client_derive(service(
    method_service_name = "token",
    method_service_fn = "wipe_token_account"
))]
pub struct TokenWipeTransaction {
    transaction: Transaction,
    services: Proto,
}

impl TokenWipeTransaction {
    pub fn new() -> TokenWipeTransaction {
        let transaction = Transaction::with_max_transaction_fee(Hbar::new(30.0));
        let services = Proto::new();
        TokenWipeTransaction {
            transaction,
            services,
        }
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

    // amount
    gen_transaction_amount_fns!();

    // serial_numbers
    gen_transaction_serial_numbers_fns!();
}

#[derive(Debug, Clone, PartialEq, TransactionProto)]
#[hedera_rust_client_derive(proto(
    proto_enum = "TokenWipe",
    proto_type = "TokenWipeAccountTransactionBody"
))]
struct Proto {
    #[hedera_rust_client_derive(to_option_proto)]
    pub token: Option<TokenId>,
    #[hedera_rust_client_derive(to_option_proto)]
    pub account: Option<AccountId>,
    pub amount: u64,
    pub serial_numbers: Vec<i64>,
}

impl Proto {
    pub fn new() -> Self {
        Proto {
            token: None,
            account: None,
            amount: 0,
            serial_numbers: Vec::new(),
        }
    }
}
