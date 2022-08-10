use hedera_rust_client_derive::{TransactionExecute, TransactionProto, TransactionSchedule};

use crate::entity_id::validate_option_id_checksum;

use crate::transaction::Transaction;
use crate::Client;
use crate::Hbar;
use crate::HederaError;
use crate::ScheduleId;

#[derive(TransactionSchedule, TransactionExecute, Debug, Clone, PartialEq)]
#[hedera_rust_client_derive(service(
    method_service_name = "schedule",
    method_service_fn = "delete_schedule"
))]
pub struct ScheduleDeleteTransaction {
    transaction: Transaction,
    services: Proto,
}

impl ScheduleDeleteTransaction {
    pub fn new() -> ScheduleDeleteTransaction {
        let transaction = Transaction::with_max_transaction_fee(Hbar::new(5.0));
        let services = Proto::new();
        ScheduleDeleteTransaction {
            transaction,
            services,
        }
    }

    fn validate_network_on_ids(&self, client: &Client) -> Result<(), HederaError> {
        validate_option_id_checksum(&self.services.schedule_id, client)?;
        Ok(())
    }

    gen_transaction_schedule_id_fns!();
}

#[derive(Debug, Clone, PartialEq, TransactionProto)]
#[hedera_rust_client_derive(proto(
    proto_enum = "ScheduleDelete",
    proto_type = "ScheduleDeleteTransactionBody"
))]
struct Proto {
    #[hedera_rust_client_derive(to_option_proto)]
    pub schedule_id: Option<ScheduleId>,
}

impl Proto {
    pub fn new() -> Self {
        Proto { schedule_id: None }
    }
}
