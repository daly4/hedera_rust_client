use chrono::Duration;
use hedera_rust_client_derive::{TransactionExecute, TransactionProto, TransactionSchedule};
use std::convert::TryFrom;

use crate::entity_id::validate_option_id_checksum;
use crate::transaction::Transaction;
use crate::utils::DEFAULT_DURATION;
use crate::AccountId;
use crate::Client;
use crate::Hbar;
use crate::InitcodeSource;
use crate::Key;
use crate::RealmId;
use crate::ShardId;
use crate::StakedId;

use crate::HederaError;

#[derive(TransactionSchedule, TransactionExecute, Debug, Clone, PartialEq)]
#[hedera_rust_client_derive(service(
    method_service_name = "contract",
    method_service_fn = "create_contract"
))]
pub struct ContractCreateTransaction {
    transaction: Transaction,
    services: Proto,
}

impl ContractCreateTransaction {
    pub fn new() -> ContractCreateTransaction {
        let transaction = Transaction::with_max_transaction_fee(Hbar::new(20.0));
        let mut services = Proto::new();
        services.auto_renew_period = Some(*DEFAULT_DURATION);
        ContractCreateTransaction {
            transaction,
            services,
        }
    }

    fn validate_network_on_ids(&self, client: &Client) -> Result<(), HederaError> {
        validate_option_id_checksum(&self.services.proxy_account_id, client)?;
        Ok(())
    }

    gen_transaction_admin_key_fns!();

    // SetAutoRenewPeriod sets the time duration for when account is charged to extend its expiration date. When the account
    // is created, the payer account is charged enough hbars so that the new account will not expire for the next
    // auto renew period. When it reaches the expiration time, the new account will then be automatically charged to
    // renew for another auto renew period. If it does not have enough hbars to renew for that long, then the  remaining
    // hbars are used to extend its expiration as long as possible. If it is has a zero balance when it expires,
    // then it is deleted.
    gen_transaction_auto_renew_period_fns!();

    gen_transaction_memo_fns!();

    gen_transaction_gas_fns!();

    gen_transaction_initial_balance_i64!();

    // constructor_params
    gen_transaction_contract_params!(
        constructor_parameters,
        constructor_parameters,
        set_constructor_parameters_raw,
        set_constructor_parameters
    );

    gen_transaction_max_automatic_token_associations_fns!();

    gen_transaction_auto_renew_account_id_fns!();

    gen_transaction_decline_award_fns!();

    gen_transaction_initcode_source_option_fns!();

    gen_transaction_staked_id_option_fns!();
}

#[derive(Debug, Clone, PartialEq, TransactionProto)]
#[hedera_rust_client_derive(proto(
    proto_enum = "ContractCreateInstance",
    proto_type = "ContractCreateTransactionBody"
))]
struct Proto {
    #[hedera_rust_client_derive(to_option_proto)]
    pub admin_key: Option<Key>,
    pub gas: i64,
    pub initial_balance: i64,
    #[hedera_rust_client_derive(to_option_proto)]
    pub proxy_account_id: Option<AccountId>,
    #[hedera_rust_client_derive(to_option_proto)]
    pub auto_renew_period: Option<Duration>,
    pub constructor_parameters: Vec<u8>,
    #[hedera_rust_client_derive(to_option_proto)]
    pub shard_id: Option<ShardId>,
    #[hedera_rust_client_derive(to_option_proto)]
    pub realm_id: Option<RealmId>,
    #[hedera_rust_client_derive(to_option_proto)]
    pub new_realm_admin_key: Option<Key>,
    pub memo: String,
    pub max_automatic_token_associations: i32,
    #[hedera_rust_client_derive(to_option_proto)]
    pub auto_renew_account_id: Option<AccountId>,
    pub decline_reward: bool,
    #[hedera_rust_client_derive(to_option_proto)]
    pub initcode_source: Option<InitcodeSource>,
    #[hedera_rust_client_derive(to_option_proto)]
    pub staked_id: Option<StakedId>,
}

impl Proto {
    pub fn new() -> Self {
        Proto {
            admin_key: None,
            gas: 0,
            initial_balance: 0,
            proxy_account_id: None,
            auto_renew_period: None,
            constructor_parameters: Vec::new(),
            shard_id: None,
            realm_id: None,
            new_realm_admin_key: None,
            memo: String::new(),
            max_automatic_token_associations: 0,
            auto_renew_account_id: None,
            decline_reward: false,
            initcode_source: None,
            staked_id: None,
        }
    }
}
