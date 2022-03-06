use crate::entity_id::validate_option_id_checksum;

use crate::key_list::KeyList;
use crate::transaction::Transaction;
use crate::Client;
use crate::FileId;
use crate::Hbar;
use crate::HederaError;
use chrono::{DateTime, Utc};
use hedera_derive::{TransactionExecute, TransactionProto, TransactionSchedule};

#[derive(TransactionSchedule, TransactionExecute, Debug, Clone)]
#[hedera_derive(service(method_service_name = "file", method_service_fn = "update_file"))]
pub struct FileUpdateTransaction {
    transaction: Transaction,
    services: Proto,
}

impl FileUpdateTransaction {
    pub fn new() -> FileUpdateTransaction {
        let transaction = Transaction::with_max_transaction_fee(Hbar::new(5.0));
        let services = Proto::new();
        FileUpdateTransaction { transaction, services }
    }

    fn validate_network_on_ids(&self, client: &Client) -> Result<(), HederaError> {
        validate_option_id_checksum(&self.services.file_id, client)?;
        Ok(())
    }

    // file_id
    gen_transaction_file_id_fns!();

    // expiration_time
    gen_transaction_expiration_time_fns!();

    // keys
    gen_transaction_keys_fns!();

    // contents
    gen_transaction_contents_fns!();

    // memo
    gen_transaction_optional_memo_fns!();
}

#[derive(Debug, Clone, TransactionProto)]
#[hedera_derive(proto(proto_enum = "FileUpdate", proto_type = "FileUpdateTransactionBody"))]
struct Proto {
    #[hedera_derive(to_option_proto)]
    pub file_id: Option<FileId>,
    #[hedera_derive(to_option_proto)]
    pub expiration_time: Option<DateTime<Utc>>,
    #[hedera_derive(to_option_proto)]
    pub keys: Option<KeyList>,
    pub contents: Vec<u8>,
    pub memo: Option<String>,
}

impl Proto {
    pub fn new() -> Self {
        Proto {
            file_id: None,
            expiration_time: None,
            keys: None,
            contents: Vec::new(),
            memo: None,
        }
    }
}
