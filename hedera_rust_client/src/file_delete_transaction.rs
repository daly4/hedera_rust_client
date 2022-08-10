use crate::entity_id::validate_option_id_checksum;

use crate::transaction::Transaction;
use crate::Client;
use crate::FileId;
use crate::Hbar;
use crate::HederaError;
use hedera_rust_client_derive::{TransactionExecute, TransactionProto, TransactionSchedule};

#[derive(TransactionSchedule, TransactionExecute, Debug, Clone, PartialEq)]
#[hedera_rust_client_derive(service(method_service_name = "file", method_service_fn = "delete_file"))]
pub struct FileDeleteTransaction {
    transaction: Transaction,
    services: Proto,
}

impl FileDeleteTransaction {
    pub fn new() -> FileDeleteTransaction {
        let transaction = Transaction::with_max_transaction_fee(Hbar::new(5.0));
        let services = Proto::new();
        FileDeleteTransaction {
            transaction,
            services,
        }
    }

    fn validate_network_on_ids(&self, client: &Client) -> Result<(), HederaError> {
        validate_option_id_checksum(&self.services.file_id, client)?;
        Ok(())
    }

    // file_id
    gen_transaction_file_id_fns!();
}

#[derive(Debug, Clone, PartialEq, TransactionProto)]
#[hedera_rust_client_derive(proto(proto_enum = "FileDelete", proto_type = "FileDeleteTransactionBody"))]
struct Proto {
    #[hedera_rust_client_derive(to_option_proto)]
    pub file_id: Option<FileId>,
}

impl Proto {
    pub fn new() -> Self {
        Proto { file_id: None }
    }
}
