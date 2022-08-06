use hedera_derive::{TransactionExecute, TransactionProto, TransactionSchedule};

use crate::entity_id::{validate_id_checksum, validate_option_id_checksum};
use crate::proto::services::contract_delete_transaction_body::Obtainers as ProtoObtainers;
use crate::proto::ToProto;
use crate::transaction::Transaction;
use crate::AccountId;
use crate::Client;
use crate::ContractId;
use crate::Hbar;
use crate::HederaError;

#[derive(TransactionSchedule, TransactionExecute, Debug, Clone)]
#[hedera_derive(service(
    method_service_name = "contract",
    method_service_fn = "delete_contract"
))]
pub struct ContractDeleteTransaction {
    transaction: Transaction,
    services: Proto,
}

impl ContractDeleteTransaction {
    pub fn new() -> ContractDeleteTransaction {
        let transaction = Transaction::with_max_transaction_fee(Hbar::new(2.0));
        let services = Proto::new();
        ContractDeleteTransaction {
            transaction,
            services,
        }
    }

    fn validate_network_on_ids(&self, client: &Client) -> Result<(), HederaError> {
        validate_option_id_checksum(&self.services.contract_id, client)?;
        if let Some(obtainers) = &self.services.obtainers {
            match obtainers {
                Obtainers::TransferAccountId(id) => validate_id_checksum(id, client)?,
                Obtainers::TransferContractId(id) => validate_id_checksum(id, client)?,
            }
        }
        Ok(())
    }

    gen_transaction_contract_id_fns!();

    // transfer_contract_id
    gen_get_failable_fn!(
        obtainers,
        ContractId,
        transfer_contract_id,
        (|v: Obtainers| match v {
            Obtainers::TransferContractId(id) => Ok(id),
            _ => Err(HederaError::InvalidSetTransferId),
        })
    );

    // set_transfer_contract_id
    // Sets the contract ID which will receive all remaining hbars.
    gen_transaction_set_failable_option_fn!(
        contract_id,
        ContractId,
        Obtainers,
        obtainers,
        set_transfer_contract_id,
        (|id: ContractId| Ok(Obtainers::TransferContractId(id)))
    );

    // transfer_account_id
    gen_get_failable_fn!(
        obtainers,
        AccountId,
        transfer_account_id,
        (|v: Obtainers| match v {
            Obtainers::TransferAccountId(id) => Ok(id),
            _ => Err(HederaError::InvalidSetTransferId),
        })
    );

    // set_transfer_account_id
    // Sets the account ID which will receive all remaining hbars.
    gen_transaction_set_failable_option_fn!(
        account_id,
        AccountId,
        Obtainers,
        obtainers,
        set_transfer_account_id,
        (|id: AccountId| Ok(Obtainers::TransferAccountId(id)))
    );
}

#[derive(Debug, Clone, TransactionProto)]
#[hedera_derive(proto(
    proto_enum = "ContractDeleteInstance",
    proto_type = "ContractDeleteTransactionBody"
))]
struct Proto {
    #[hedera_derive(to_option_proto)]
    pub contract_id: Option<ContractId>,
    #[hedera_derive(to_proto_with_fn = "obtainer_to_proto")]
    pub obtainers: Option<Obtainers>,
}

impl Proto {
    pub fn new() -> Self {
        Proto {
            contract_id: None,
            obtainers: None,
        }
    }
}

#[derive(Debug, Clone)]
enum Obtainers {
    TransferAccountId(AccountId),
    TransferContractId(ContractId),
}

fn obtainer_to_proto(o: &Option<Obtainers>) -> Result<Option<ProtoObtainers>, HederaError> {
    match o {
        Some(o) => {
            let services = match o {
                Obtainers::TransferAccountId(id) => {
                    ProtoObtainers::TransferAccountId(id.to_proto()?)
                }
                Obtainers::TransferContractId(id) => {
                    ProtoObtainers::TransferContractId(id.to_proto()?)
                }
            };
            Ok(Some(services))
        }
        None => Ok(None),
    }
}
