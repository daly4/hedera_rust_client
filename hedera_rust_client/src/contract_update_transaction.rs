use chrono::{DateTime, Duration, Utc};
use hedera_rust_client_derive::{TransactionExecute, TransactionProto, TransactionSchedule};

use crate::entity_id::validate_option_id_checksum;
use crate::memo::check_memo_length;
use crate::proto::services::{self};
use crate::transaction::Transaction;
use crate::AccountId;
use crate::Client;
use crate::ContractId;
use crate::FileId;
use crate::Hbar;
use crate::HederaError;
use crate::Key;
use crate::StakedId;

#[derive(TransactionSchedule, TransactionExecute, Debug, Clone, PartialEq)]
#[hedera_rust_client_derive(service(
    method_service_name = "contract",
    method_service_fn = "update_contract"
))]
pub struct ContractUpdateTransaction {
    transaction: Transaction,
    services: Proto,
}

impl ContractUpdateTransaction {
    pub fn new() -> ContractUpdateTransaction {
        let transaction = Transaction::with_max_transaction_fee(Hbar::new(2.0));
        let services = Proto::new();
        ContractUpdateTransaction {
            transaction,
            services,
        }
    }

    fn validate_network_on_ids(&self, client: &Client) -> Result<(), HederaError> {
        validate_option_id_checksum(&self.services.contract_id, client)?;
        Ok(())
    }

    // contract_id
    gen_transaction_contract_id_fns!();

    // bytecode_file_id
    gen_transaction_bytecode_file_id_fns!();

    // admin_key
    gen_transaction_admin_key_fns!();

    // proxy_account_id
    gen_transaction_proxy_account_id_fns!();

    // auto_renew_period
    gen_transaction_auto_renew_period_fns!();

    // set_expiration_time
    // set_expiration_time extends the expiration of the instance and its account to the provIDed time. If the time provIDed
    // is the current or past time, then there will be no effect.
    gen_transaction_expiration_time_fns!();

    gen_transaction_max_automatic_token_associations_option_fns!();

    gen_transaction_auto_renew_account_id_fns!();

    gen_transaction_decline_award_option_fns!();

    // contract_memo
    gen_get_proto_option_fn!(memo_field, String, contract_memo);

    // set_contract_memo
    gen_transaction_set_failable_fn!(
        contract_memo,
        String,
        memo_field,
        set_contract_memo,
        (|contract_memo: String| {
            if let Err(e) = check_memo_length(&contract_memo) {
                return Err(e);
            }
            Ok(Some(contract_memo))
        })
    );

    gen_transaction_staked_id_option_fns!();
}

#[derive(Debug, Clone, PartialEq, TransactionProto)]
#[hedera_rust_client_derive(proto(
    proto_enum = "ContractUpdateInstance",
    proto_type = "ContractUpdateTransactionBody"
))]
struct Proto {
    #[hedera_rust_client_derive(to_option_proto)]
    pub contract_id: Option<ContractId>,
    #[hedera_rust_client_derive(to_option_proto)]
    pub expiration_time: Option<DateTime<Utc>>,
    #[hedera_rust_client_derive(to_option_proto)]
    pub admin_key: Option<Key>,
    #[hedera_rust_client_derive(to_option_proto)]
    pub proxy_account_id: Option<AccountId>,
    #[hedera_rust_client_derive(to_option_proto)]
    pub auto_renew_period: Option<Duration>,
    #[hedera_rust_client_derive(to_option_proto)]
    pub file_id: Option<FileId>,
    pub max_automatic_token_associations: Option<i32>,
    #[hedera_rust_client_derive(to_option_proto)]
    pub auto_renew_account_id: Option<AccountId>,
    pub decline_reward: Option<bool>,
    #[hedera_rust_client_derive(to_proto_with_fn = "memo_proto")]
    pub memo_field: Option<String>,
    #[hedera_rust_client_derive(to_option_proto)]
    pub staked_id: Option<StakedId>,
}

impl Proto {
    pub fn new() -> Proto {
        Proto {
            contract_id: None,
            expiration_time: None,
            admin_key: None,
            proxy_account_id: None,
            auto_renew_period: None,
            file_id: None,
            memo_field: None,
            max_automatic_token_associations: None,
            auto_renew_account_id: None,
            decline_reward: None,
            staked_id: None,
        }
    }
}

fn memo_proto(
    memo_field: &Option<String>,
) -> Result<Option<services::contract_update_transaction_body::MemoField>, HederaError> {
    let f = memo_field
        .as_ref()
        .map(|x| services::contract_update_transaction_body::MemoField::MemoWrapper(x.clone()));
    Ok(f)
}
