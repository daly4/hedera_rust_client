use crate::entity_id::validate_option_id_checksum;

use crate::transaction::Transaction;
use crate::AccountId;
use crate::Client;
use crate::Hbar;
use crate::HederaError;
use crate::Key;
use crate::TopicId;
use chrono::{DateTime, Utc, Duration};
use hedera_derive::{TransactionExecute, TransactionProto, TransactionSchedule};

#[derive(TransactionSchedule, TransactionExecute, Debug, Clone)]
#[hedera_derive(service(method_service_name = "topic", method_service_fn = "update_topic"))]
pub struct TopicUpdateTransaction {
    transaction: Transaction,
    services: Proto,
}

impl TopicUpdateTransaction {
    pub fn new() -> TopicUpdateTransaction {
        let transaction = Transaction::with_max_transaction_fee(Hbar::new(2.0));
        let services = Proto::new();
        TopicUpdateTransaction { transaction, services }
    }

    fn validate_network_on_ids(&self, client: &Client) -> Result<(), HederaError> {
        validate_option_id_checksum(&self.services.topic_id, client)?;
        validate_option_id_checksum(&self.services.auto_renew_account, client)?;
        Ok(())
    }

    // topic_id
    gen_transaction_topic_id_fns!();

    // memo
    gen_transaction_optional_memo_fns!();

    // expiration_time
    gen_transaction_expiration_time_fns!();

    // admin_key
    gen_transaction_admin_key_fns!();

    // submit_key
    gen_transaction_submit_key_fns!();

    // auto_renew_account
    gen_transaction_auto_renew_account_fns!();

    // auto_renew_period
    gen_transaction_auto_renew_period_fns!();
}

#[derive(Debug, Clone, TransactionProto)]
#[hedera_derive(proto(
    proto_enum = "ConsensusUpdateTopic",
    proto_type = "ConsensusUpdateTopicTransactionBody"
))]
struct Proto {
    #[hedera_derive(to_option_proto)]
    pub topic_id: Option<TopicId>,
    pub memo: Option<String>,
    #[hedera_derive(to_option_proto)]
    pub expiration_time: Option<DateTime<Utc>>,
    #[hedera_derive(to_option_proto)]
    pub admin_key: Option<Key>,
    #[hedera_derive(to_option_proto)]
    pub submit_key: Option<Key>,
    #[hedera_derive(to_option_proto)]
    pub auto_renew_period: Option<Duration>,
    #[hedera_derive(to_option_proto)]
    pub auto_renew_account: Option<AccountId>,
}

impl Proto {
    pub fn new() -> Self {
        Proto {
            topic_id: None,
            memo: None,
            expiration_time: None,
            admin_key: None,
            submit_key: None,
            auto_renew_account: None,
            auto_renew_period: None,
        }
    }
}
