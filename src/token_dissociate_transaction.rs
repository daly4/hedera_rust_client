use crate::entity_id::{validate_id_checksum, validate_option_id_checksum};

use crate::transaction::Transaction;
use crate::AccountId;
use crate::Client;
use crate::Hbar;
use crate::HederaError;
use crate::TokenId;
use hedera_derive::{TransactionExecute, TransactionProto, TransactionSchedule};

#[derive(TransactionSchedule, TransactionExecute, Debug, Clone)]
#[hedera_derive(service(method_service_name = "token", method_service_fn = "dissociate_tokens"))]
pub struct TokenDissociateTransaction {
    transaction: Transaction,
    services: Proto,
}

impl TokenDissociateTransaction {
    pub fn new() -> TokenDissociateTransaction {
        let transaction = Transaction::with_max_transaction_fee(Hbar::new(5.0));
        let services = Proto::new();
        TokenDissociateTransaction { transaction, services }
    }

    fn validate_network_on_ids(&self, client: &Client) -> Result<(), HederaError> {
        validate_option_id_checksum(&self.services.account, client)?;
        for id in self.services.tokens.iter() {
            validate_id_checksum(id, client)?;
        }
        Ok(())
    }

    // account_id
    gen_transaction_account_fns!();

    // tokens
    gen_transaction_tokens_fns!();
}

#[derive(Debug, Clone, TransactionProto)]
#[hedera_derive(proto(
    proto_enum = "TokenDissociate",
    proto_type = "TokenDissociateTransactionBody"
))]
struct Proto {
    #[hedera_derive(to_option_proto)]
    pub account: Option<AccountId>,
    #[hedera_derive(to_proto_vec = "TokenId")]
    pub tokens: Vec<TokenId>,
}

impl Proto {
    pub fn new() -> Self {
        Proto {
            account: None,
            tokens: Vec::new(),
        }
    }
}
