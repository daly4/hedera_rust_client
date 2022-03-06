use hedera_derive::{TransactionExecute, TransactionProto, TransactionSchedule};
use std::convert::TryFrom;

use crate::entity_id::validate_option_id_checksum;
use crate::transaction::Transaction;
use crate::Client;
use crate::ContractId;
use crate::Hbar;
use crate::HederaError;

#[derive(TransactionSchedule, TransactionExecute, Debug, Clone)]
#[hedera_derive(service(
    method_service_name = "contract",
    method_service_fn = "contract_call_method"
))]
pub struct ContractExecuteTransaction {
    transaction: Transaction,
    services: Proto,
}

impl ContractExecuteTransaction {
    pub fn new() -> ContractExecuteTransaction {
        let transaction = Transaction::with_max_transaction_fee(Hbar::new(2.0));
        let services = Proto::new();
        ContractExecuteTransaction { transaction, services }
    }

    fn validate_network_on_ids(&self, client: &Client) -> Result<(), HederaError> {
        validate_option_id_checksum(&self.services.contract_id, client)?;
        Ok(())
    }

    // contract_id
    gen_transaction_contract_id_fns!();

    // gas
    gen_transaction_gas_fns!();

    // payable_amount
    gen_transaction_get_set_with_hbar_i64!(amount, payable_amount, set_payable_amount);

    // function_parameters
    gen_transaction_contract_params!(
        function_parameters,
        function_parameters,
        set_function_parameters,
        set_function
    );
}

#[derive(Debug, Clone, TransactionProto)]
#[hedera_derive(proto(
    proto_enum = "ContractCall",
    proto_type = "ContractCallTransactionBody"
))]
struct Proto {
    #[hedera_derive(to_option_proto)]
    pub contract_id: Option<ContractId>,
    pub gas: i64,
    pub amount: i64,
    pub function_parameters: Vec<u8>,
}

impl Proto {
    pub fn new() -> Proto {
        Proto {
            contract_id: None,
            gas: 0,
            amount: 0,
            function_parameters: Vec::new(),
        }
    }
}
