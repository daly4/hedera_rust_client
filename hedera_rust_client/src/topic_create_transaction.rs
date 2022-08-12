use crate::entity_id::validate_option_id_checksum;

use crate::transaction::Transaction;
use crate::utils::DEFAULT_AUTO_RENEW_PERIOD;
use crate::AccountId;
use crate::Client;
use crate::Hbar;
use crate::HederaError;
use crate::Key;
use chrono::Duration;
use hedera_rust_client_derive::{TransactionExecute, TransactionProto, TransactionSchedule};

#[derive(TransactionSchedule, TransactionExecute, Debug, Clone, PartialEq)]
#[hedera_rust_client_derive(service(
    method_service_name = "topic",
    method_service_fn = "create_topic"
))]
pub struct TopicCreateTransaction {
    transaction: Transaction,
    services: Proto,
}

impl TopicCreateTransaction {
    pub fn new() -> TopicCreateTransaction {
        let transaction = Transaction::with_max_transaction_fee(Hbar::new(2.0));
        let mut services = Proto::new();
        services.auto_renew_period = Some(*DEFAULT_AUTO_RENEW_PERIOD);
        TopicCreateTransaction {
            transaction,
            services,
        }
    }

    fn validate_network_on_ids(&self, client: &Client) -> Result<(), HederaError> {
        validate_option_id_checksum(&self.services.auto_renew_account, client)?;
        Ok(())
    }

    // memo
    gen_transaction_memo_fns!();

    // admin_key
    gen_transaction_admin_key_fns!();

    // submit_key
    gen_transaction_submit_key_fns!();

    // auto_renew_account
    gen_transaction_auto_renew_account_fns!();

    // auto_renew_period
    gen_transaction_auto_renew_period_fns!();
}

#[derive(Debug, Clone, PartialEq, TransactionProto)]
#[hedera_rust_client_derive(proto(
    proto_enum = "ConsensusCreateTopic",
    proto_type = "ConsensusCreateTopicTransactionBody"
))]
struct Proto {
    pub memo: String,
    #[hedera_rust_client_derive(to_option_proto)]
    pub admin_key: Option<Key>,
    #[hedera_rust_client_derive(to_option_proto)]
    pub submit_key: Option<Key>,
    #[hedera_rust_client_derive(to_option_proto)]
    pub auto_renew_period: Option<Duration>,
    #[hedera_rust_client_derive(to_option_proto)]
    pub auto_renew_account: Option<AccountId>,
}

impl Proto {
    pub fn new() -> Self {
        Proto {
            memo: String::new(),
            admin_key: None,
            submit_key: None,
            auto_renew_account: None,
            auto_renew_period: None,
        }
    }
}
