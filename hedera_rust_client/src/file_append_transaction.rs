use crate::entity_id::validate_option_id_checksum;
use crate::proto::{services, ToProto};
use crate::transaction::Transaction;
use crate::Client;
use crate::FileId;
use crate::Hbar;
use crate::HederaError;
use hedera_rust_client_derive::{TransactionChunked, TransactionChunkedSchedule, TransactionProto};

const CHUNK_SIZE: usize = 1024;

#[derive(TransactionChunkedSchedule, TransactionChunked, Debug, Clone, PartialEq)]
#[hedera_rust_client_derive(service(method_service_name = "file", method_service_fn = "append_content"))]
pub struct FileAppendTransaction {
    transaction: Transaction,
    services: Proto,
    max_chunks: usize,
    data: Vec<u8>,
}

impl FileAppendTransaction {
    pub fn new() -> FileAppendTransaction {
        let transaction = Transaction::with_max_transaction_fee(Hbar::new(5.0));
        let services = Proto::new();
        FileAppendTransaction {
            transaction,
            services,
            max_chunks: 20,
            data: Vec::new(),
        }
    }

    fn validate_network_on_ids(&self, client: &Client) -> Result<(), HederaError> {
        validate_option_id_checksum(&self.services.file_id, client)?;
        Ok(())
    }

    // topic_id
    gen_transaction_file_id_fns!();

    // max_chunks
    gen_transaction_get_set_fns!(max_chunks, usize, max_chunks, set_max_chunks);

    // contents
    gen_transaction_get_set_fns!(data, Vec<u8>, contents, set_contents);

    fn on_freeze_chunk(
        &mut self,
        _initial_transaction_id: Option<services::TransactionId>,
        start_index: usize,
        end_index: usize,
        _chunk: usize,
        _total: usize,
    ) -> Result<(), HederaError> {
        self.services.contents = self.data[start_index..end_index].to_vec();
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, TransactionProto)]
#[hedera_rust_client_derive(proto(proto_enum = "FileAppend", proto_type = "FileAppendTransactionBody"))]
struct Proto {
    #[hedera_rust_client_derive(to_option_proto)]
    pub file_id: Option<FileId>,
    pub contents: Vec<u8>,
}

impl Proto {
    pub fn new() -> Self {
        Proto {
            file_id: None,
            contents: Vec::new(),
        }
    }
}
