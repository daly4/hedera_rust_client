use hedera_derive::{TransactionExecute, TransactionProto};

use crate::entity_id::validate_option_id_checksum;

use crate::transaction::Transaction;
use crate::AccountId;
use crate::Client;
use crate::Hbar;
use crate::HederaError;

#[derive(TransactionExecute, Debug, Clone, PartialEq)]
#[hedera_derive(service(method_service_name = "crypto", method_service_fn = "delete_live_hash"))]
pub struct LiveHashDeleteTransaction {
    transaction: Transaction,
    services: Proto,
}

impl LiveHashDeleteTransaction {
    pub fn new() -> LiveHashDeleteTransaction {
        let transaction = Transaction::with_max_transaction_fee(Hbar::new(2.0));
        let services = Proto::new();
        LiveHashDeleteTransaction {
            transaction,
            services,
        }
    }

    fn validate_network_on_ids(&self, client: &Client) -> Result<(), HederaError> {
        validate_option_id_checksum(&self.services.account_of_live_hash, client)?;
        Ok(())
    }

    // account_of_live_hash
    gen_transaction_account_of_live_hash_fns!();

    // live_hash_to_delete
    gen_transaction_live_hash_to_delete_fns!();
}

#[derive(Debug, Clone, PartialEq, TransactionProto)]
#[hedera_derive(proto(
    proto_enum = "CryptoDeleteLiveHash",
    proto_type = "CryptoDeleteLiveHashTransactionBody"
))]
struct Proto {
    #[hedera_derive(to_option_proto)]
    pub account_of_live_hash: Option<AccountId>,
    pub live_hash_to_delete: Vec<u8>,
}

impl Proto {
    pub fn new() -> Self {
        Proto {
            account_of_live_hash: None,
            live_hash_to_delete: Vec::new(),
        }
    }
}
