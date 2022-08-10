use crate::entity_id::validate_option_id_checksum;
use crate::proto::{services, ToProto};
use crate::transaction::Transaction;
use crate::Client;
use crate::Hbar;
use crate::HederaError;
use crate::TopicId;
use hedera_rust_client_derive::{TransactionChunked, TransactionChunkedSchedule, TransactionProto};

const CHUNK_SIZE: usize = 1024;

#[derive(TransactionChunkedSchedule, TransactionChunked, Debug, Clone, PartialEq)]
#[hedera_rust_client_derive(service(method_service_name = "topic", method_service_fn = "submit_message"))]
pub struct TopicMessageSubmitTransaction {
    transaction: Transaction,
    services: Proto,
    max_chunks: usize,
    data: Vec<u8>,
}

impl TopicMessageSubmitTransaction {
    pub fn new() -> TopicMessageSubmitTransaction {
        let transaction = Transaction::with_max_transaction_fee(Hbar::new(2.0));
        let services = Proto::new();
        TopicMessageSubmitTransaction {
            transaction,
            services,
            max_chunks: 20,
            data: Vec::new(),
        }
    }

    fn validate_network_on_ids(&self, client: &Client) -> Result<(), HederaError> {
        validate_option_id_checksum(&self.services.topic_id, client)?;
        Ok(())
    }

    // topic_id
    gen_transaction_topic_id_fns!();

    // max_chunks
    gen_transaction_get_set_fns!(max_chunks, usize, max_chunks, set_max_chunks);

    // message
    gen_transaction_get_set_fns!(data, Vec<u8>, message, set_message);

    fn on_freeze_chunk(
        &mut self,
        initial_transaction_id: Option<services::TransactionId>,
        start_index: usize,
        end_index: usize,
        chunk: usize,
        total: usize,
    ) -> Result<(), HederaError> {
        self.services.message = self.data[start_index..end_index].to_vec();
        if total != 1 {
            self.services.chunk_info = Some(services::ConsensusMessageChunkInfo {
                initial_transaction_id,
                total: total as i32,
                number: chunk as i32 + 1,
            });
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, TransactionProto)]
#[hedera_rust_client_derive(proto(
    proto_enum = "ConsensusSubmitMessage",
    proto_type = "ConsensusSubmitMessageTransactionBody"
))]
struct Proto {
    #[hedera_rust_client_derive(to_option_proto)]
    pub topic_id: Option<TopicId>,
    pub message: Vec<u8>,
    pub chunk_info: Option<services::ConsensusMessageChunkInfo>,
}

impl Proto {
    pub fn new() -> Self {
        Proto {
            topic_id: None,
            message: Vec::new(),
            chunk_info: None,
        }
    }
}
