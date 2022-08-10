use chrono::{DateTime, Duration, Utc};
use hedera_derive::{TransactionExecute, TransactionProto, TransactionSchedule};

use crate::key_list::KeyList;
use crate::transaction::Transaction;
use crate::Client;
use crate::Hbar;
use crate::HederaError;
use crate::Key;
use crate::RealmId;
use crate::ShardId;

#[derive(TransactionSchedule, TransactionExecute, Debug, Clone, PartialEq)]
#[hedera_derive(service(method_service_name = "file", method_service_fn = "create_file"))]
pub struct FileCreateTransaction {
    transaction: Transaction,
    services: Proto,
}

impl FileCreateTransaction {
    pub fn new() -> FileCreateTransaction {
        let transaction = Transaction::with_max_transaction_fee(Hbar::new(5.0));
        let mut services = Proto::new();
        services.expiration_time = Some(Utc::now() + Duration::seconds(7890000));
        FileCreateTransaction {
            transaction,
            services,
        }
    }

    fn validate_network_on_ids(&self, _client: &Client) -> Result<(), HederaError> {
        Ok(())
    }

    // expiration_time
    gen_transaction_expiration_time_fns!();

    // keys
    gen_transaction_keys_fns!();

    // contents
    gen_transaction_contents_fns!();

    // memo
    gen_transaction_memo_fns!();
}

#[derive(Debug, Clone, PartialEq, TransactionProto)]
#[hedera_derive(proto(proto_enum = "FileCreate", proto_type = "FileCreateTransactionBody"))]
struct Proto {
    #[hedera_derive(to_option_proto)]
    pub expiration_time: Option<DateTime<Utc>>,
    #[hedera_derive(to_option_proto)]
    pub keys: Option<KeyList>,
    pub contents: Vec<u8>,
    #[hedera_derive(to_option_proto)]
    pub shard_id: Option<ShardId>,
    #[hedera_derive(to_option_proto)]
    pub realm_id: Option<RealmId>,
    #[hedera_derive(to_option_proto)]
    pub new_realm_admin_key: Option<Key>,
    pub memo: String,
}

impl Proto {
    pub fn new() -> Self {
        Proto {
            expiration_time: None,
            keys: None,
            contents: Vec::new(),
            shard_id: None,            // ignored
            realm_id: None,            // ignored
            new_realm_admin_key: None, // ignored
            memo: String::new(),
        }
    }
}
