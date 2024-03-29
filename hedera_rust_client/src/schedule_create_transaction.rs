use hedera_rust_client_derive::{TransactionExecute, TransactionProto, TransactionSchedule};

use crate::entity_id::validate_option_id_checksum;
use crate::error::HederaError;
use crate::proto::services::{self};
use crate::transaction::Transaction;
use crate::AccountId;
use crate::Client;
use crate::Hbar;
use crate::Key;

#[derive(TransactionSchedule, TransactionExecute, Debug, Clone, PartialEq)]
#[hedera_rust_client_derive(service(
    method_service_name = "schedule",
    method_service_fn = "create_schedule"
))]
pub struct ScheduleCreateTransaction {
    transaction: Transaction,
    services: Proto,
}

impl ScheduleCreateTransaction {
    pub fn new() -> ScheduleCreateTransaction {
        let transaction = Transaction::with_max_transaction_fee(Hbar::new(5.0));
        let services = Proto::new();
        ScheduleCreateTransaction {
            transaction,
            services,
        }
    }

    fn validate_network_on_ids(&self, client: &Client) -> Result<(), HederaError> {
        validate_option_id_checksum(&self.services.payer_account_id, client)?;
        Ok(())
    }

    pub fn set_schedulable_transaction_body(
        &mut self,
        tx_body: services::SchedulableTransactionBody,
    ) -> Result<&mut Self, HederaError> {
        self.require_not_frozen()?;
        self.services.scheduled_transaction_body = Some(tx_body);
        Ok(self)
    }

    gen_transaction_payer_account_id_fns!();

    gen_transaction_admin_key_fns!();

    gen_transaction_memo_fns!();
}

#[derive(Debug, Clone, PartialEq, TransactionProto)]
#[hedera_rust_client_derive(proto(
    proto_enum = "ScheduleCreate",
    proto_type = "ScheduleCreateTransactionBody"
))]
struct Proto {
    pub scheduled_transaction_body: Option<services::SchedulableTransactionBody>,
    pub memo: String,
    #[hedera_rust_client_derive(to_option_proto)]
    pub admin_key: Option<Key>,
    #[hedera_rust_client_derive(to_option_proto)]
    pub payer_account_id: Option<AccountId>,
    pub expiration_time: Option<services::Timestamp>,
    pub wait_for_expiry: bool,
}

impl Proto {
    pub fn new() -> Self {
        Proto {
            scheduled_transaction_body: None,
            memo: "".to_string(),
            admin_key: None,
            payer_account_id: None,
            expiration_time: None,
            wait_for_expiry: false,
        }
    }
}
