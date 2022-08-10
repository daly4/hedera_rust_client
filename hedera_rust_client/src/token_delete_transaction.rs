use crate::entity_id::validate_option_id_checksum;

use crate::transaction::Transaction;
use crate::Client;
use crate::Hbar;
use crate::HederaError;
use crate::TokenId;
use hedera_rust_client_derive::{TransactionExecute, TransactionProto, TransactionSchedule};

#[derive(TransactionSchedule, TransactionExecute, Debug, Clone, PartialEq)]
#[hedera_rust_client_derive(service(method_service_name = "token", method_service_fn = "delete_token"))]
pub struct TokenDeleteTransaction {
    transaction: Transaction,
    services: Proto,
}

impl TokenDeleteTransaction {
    pub fn new() -> TokenDeleteTransaction {
        let transaction = Transaction::with_max_transaction_fee(Hbar::new(30.0));
        let services = Proto::new();
        TokenDeleteTransaction {
            transaction,
            services,
        }
    }

    fn validate_network_on_ids(&self, client: &Client) -> Result<(), HederaError> {
        validate_option_id_checksum(&self.services.token, client)?;
        Ok(())
    }

    // token
    gen_transaction_token_fns!();
}

#[derive(Debug, Clone, PartialEq, TransactionProto)]
#[hedera_rust_client_derive(proto(
    proto_enum = "TokenDeletion",
    proto_type = "TokenDeleteTransactionBody"
))]
struct Proto {
    #[hedera_rust_client_derive(to_option_proto)]
    pub token: Option<TokenId>,
}

impl Proto {
    pub fn new() -> Self {
        Proto { token: None }
    }
}
