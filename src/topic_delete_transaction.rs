use crate::entity_id::validate_option_id_checksum;

use crate::transaction::Transaction;
use crate::Client;
use crate::Hbar;
use crate::HederaError;
use crate::TopicId;
use hedera_derive::{TransactionExecute, TransactionProto, TransactionSchedule};

#[derive(TransactionSchedule, TransactionExecute, Debug, Clone)]
#[hedera_derive(service(method_service_name = "topic", method_service_fn = "delete_topic"))]
pub struct TopicDeleteTransaction {
    transaction: Transaction,
    services: Proto,
}

impl TopicDeleteTransaction {
    pub fn new() -> TopicDeleteTransaction {
        let transaction = Transaction::with_max_transaction_fee(Hbar::new(2.0));
        let services = Proto::new();
        TopicDeleteTransaction { transaction, services }
    }

    fn validate_network_on_ids(&self, client: &Client) -> Result<(), HederaError> {
        validate_option_id_checksum(&self.services.topic_id, client)?;
        Ok(())
    }

    // topic_id
    gen_transaction_topic_id_fns!();
}

#[derive(Debug, Clone, TransactionProto)]
#[hedera_derive(proto(
    proto_enum = "ConsensusDeleteTopic",
    proto_type = "ConsensusDeleteTopicTransactionBody"
))]
struct Proto {
    #[hedera_derive(to_option_proto)]
    pub topic_id: Option<TopicId>,
}

impl Proto {
    pub fn new() -> Self {
        Proto { topic_id: None }
    }
}
