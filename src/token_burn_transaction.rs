use crate::entity_id::validate_option_id_checksum;

use crate::transaction::Transaction;
use crate::Client;
use crate::Hbar;
use crate::HederaError;
use crate::TokenId;
use hedera_derive::{TransactionExecute, TransactionProto, TransactionSchedule};

#[derive(TransactionSchedule, TransactionExecute, Debug, Clone)]
#[hedera_derive(service(method_service_name = "token", method_service_fn = "burn_token"))]
pub struct TokenBurnTransaction {
    transaction: Transaction,
    services: Proto,
}

impl TokenBurnTransaction {
    pub fn new() -> TokenBurnTransaction {
        let transaction = Transaction::with_max_transaction_fee(Hbar::new(2.0));
        let services = Proto::new();
        TokenBurnTransaction {
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

    // amount
    gen_transaction_amount_fns!();

    // serial_numbers
    gen_transaction_serial_numbers_fns!();
}

#[derive(Debug, Clone, TransactionProto)]
#[hedera_derive(proto(proto_enum = "TokenBurn", proto_type = "TokenBurnTransactionBody"))]
struct Proto {
    #[hedera_derive(to_option_proto)]
    pub token: Option<TokenId>,
    pub amount: u64,
    pub serial_numbers: Vec<i64>,
}

impl Proto {
    pub fn new() -> Self {
        Proto {
            token: None,
            amount: 0,
            serial_numbers: Vec::new(),
        }
    }
}
