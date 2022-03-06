use hedera_derive::{TransactionExecute, TransactionProto, TransactionSchedule};

use crate::entity_id::validate_option_id_checksum;

use crate::transaction::Transaction;
use crate::Client;
use crate::Hbar;
use crate::HederaError;
use crate::ScheduleId;

#[derive(TransactionSchedule, TransactionExecute, Debug, Clone)]
#[hedera_derive(service(method_service_name = "schedule", method_service_fn = "sign_schedule"))]
pub struct ScheduleSignTransaction {
    transaction: Transaction,
    services: Proto,
}

impl ScheduleSignTransaction {
    pub fn new() -> ScheduleSignTransaction {
        let transaction = Transaction::with_max_transaction_fee(Hbar::new(5.0));
        let services = Proto::new();
        ScheduleSignTransaction { transaction, services }
    }

    fn validate_network_on_ids(&self, client: &Client) -> Result<(), HederaError> {
        validate_option_id_checksum(&self.services.schedule_id, client)?;
        Ok(())
    }

    gen_transaction_schedule_id_fns!();
}

#[derive(Debug, Clone, TransactionProto)]
#[hedera_derive(proto(
    proto_enum = "ScheduleSign",
    proto_type = "ScheduleSignTransactionBody"
))]
struct Proto {
    #[hedera_derive(to_option_proto)]
    pub schedule_id: Option<ScheduleId>,
}

impl Proto {
    pub fn new() -> Self {
        Proto { schedule_id: None }
    }
}
