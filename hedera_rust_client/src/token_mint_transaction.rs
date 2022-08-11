use crate::entity_id::validate_option_id_checksum;

use crate::transaction::Transaction;
use crate::Client;
use crate::Hbar;
use crate::HederaError;
use crate::TokenId;
use hedera_rust_client_derive::{TransactionExecute, TransactionProto, TransactionSchedule};

#[derive(TransactionSchedule, TransactionExecute, Debug, Clone, PartialEq)]
#[hedera_rust_client_derive(service(
    method_service_name = "token",
    method_service_fn = "mint_token"
))]
pub struct TokenMintTransaction {
    transaction: Transaction,
    services: Proto,
}

impl TokenMintTransaction {
    pub fn new() -> TokenMintTransaction {
        let transaction = Transaction::with_max_transaction_fee(Hbar::new(30.0));
        let services = Proto::new();
        TokenMintTransaction {
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

    // metadata
    gen_transaction_metadatas_fns!();
}

#[derive(Debug, Clone, PartialEq, TransactionProto)]
#[hedera_rust_client_derive(proto(
    proto_enum = "TokenMint",
    proto_type = "TokenMintTransactionBody"
))]
struct Proto {
    #[hedera_rust_client_derive(to_option_proto)]
    pub token: Option<TokenId>,
    pub amount: u64,
    pub metadata: Vec<Vec<u8>>,
}

impl Proto {
    pub fn new() -> Self {
        Proto {
            token: None,
            amount: 0,
            metadata: Vec::new(),
        }
    }
}
