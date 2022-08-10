use hedera_derive::{TransactionExecute, TransactionProto};

use crate::transaction::Transaction;
use crate::Client;
use crate::Hbar;
use crate::HederaError;
use crate::LiveHash;

#[derive(TransactionExecute, Debug, Clone, PartialEq)]
#[hedera_derive(service(method_service_name = "crypto", method_service_fn = "add_live_hash"))]
pub struct LiveHashAddTransaction {
    transaction: Transaction,
    services: Proto,
}

impl LiveHashAddTransaction {
    pub fn new() -> LiveHashAddTransaction {
        let transaction = Transaction::with_max_transaction_fee(Hbar::new(2.0));
        let services = Proto::new();
        LiveHashAddTransaction {
            transaction,
            services,
        }
    }

    fn validate_network_on_ids(&self, _client: &Client) -> Result<(), HederaError> {
        Ok(())
    }

    // live_hash
    gen_transaction_live_hash_fns!();
}

#[derive(Debug, Clone, PartialEq, TransactionProto)]
#[hedera_derive(proto(
    proto_enum = "CryptoAddLiveHash",
    proto_type = "CryptoAddLiveHashTransactionBody"
))]
struct Proto {
    #[hedera_derive(to_option_proto)]
    pub live_hash: Option<LiveHash>,
}

impl Proto {
    pub fn new() -> Self {
        Proto { live_hash: None }
    }
}
