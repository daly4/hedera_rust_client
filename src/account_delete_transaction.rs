use crate::entity_id::validate_option_id_checksum;
use crate::transaction::Transaction;
use crate::AccountId;
use crate::Client;
use crate::Hbar;
use crate::HederaError;
use hedera_derive::{TransactionExecute, TransactionProto, TransactionSchedule};

#[derive(TransactionSchedule, TransactionExecute, Debug, Clone)]
#[hedera_derive(service(method_service_name = "crypto", method_service_fn = "crypto_delete"))]
pub struct AccountDeleteTransaction {
    transaction: Transaction,
    services: Proto,
}

impl AccountDeleteTransaction {
    pub fn new() -> AccountDeleteTransaction {
        let transaction = Transaction::with_max_transaction_fee(Hbar::new(2.0));
        let services = Proto::new();
        AccountDeleteTransaction {
            transaction,
            services,
        }
    }

    fn validate_network_on_ids(&self, client: &Client) -> Result<(), HederaError> {
        validate_option_id_checksum(&self.services.transfer_account_id, client)?;
        validate_option_id_checksum(&self.services.delete_account_id, client)?;
        Ok(())
    }

    gen_transaction_delete_account_id_fns!();

    gen_transaction_transfer_account_id_fns!();
}

#[derive(Debug, Clone, TransactionProto)]
#[hedera_derive(proto(
    proto_enum = "CryptoDelete",
    proto_type = "CryptoDeleteTransactionBody"
))]
struct Proto {
    #[hedera_derive(to_option_proto)]
    pub transfer_account_id: Option<AccountId>,
    #[hedera_derive(to_option_proto)]
    pub delete_account_id: Option<AccountId>,
}

impl Proto {
    pub fn new() -> Self {
        Proto {
            transfer_account_id: None,
            delete_account_id: None,
        }
    }
}
