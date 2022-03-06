use hedera_derive::{TransactionExecute, TransactionProto, TransactionSchedule};
use std::convert::TryFrom;
use chrono::Duration;

use crate::entity_id::validate_option_id_checksum;
use crate::transaction::Transaction;
use crate::AccountId;
use crate::Client;
use crate::FileId;
use crate::Hbar;
use crate::Key;
use crate::RealmId;
use crate::ShardId;
use crate::utils::DEFAULT_DURATION;

use crate::HederaError;

#[derive(TransactionSchedule, TransactionExecute, Debug, Clone)]
#[hedera_derive(service(
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
        ContractCreateTransaction { transaction, services }
    }

    fn validate_network_on_ids(&self, client: &Client) -> Result<(), HederaError> {
        validate_option_id_checksum(&self.services.file_id, client)?;
        validate_option_id_checksum(&self.services.proxy_account_id, client)?;
        Ok(())
    }

    // bytecode_file_id
    gen_transaction_bytecode_file_id_fns!();

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

    // SetProxyAccountID sets the ID of the account to which this account is proxy staked. If proxyAccountID is not set,
    // is an invalID account, or is an account that isn't a node, then this account is automatically proxy staked to a node
    // chosen by the network, but without earning payments. If the proxyAccountID account refuses to accept proxy staking ,
    // or if it is not currently running a node, then it will behave as if proxyAccountID was not set.
    gen_transaction_proxy_account_id_fns!();

    // constructor_params
    gen_transaction_contract_params!(
        constructor_parameters,
        constructor_parameters,
        set_constructor_parameters_raw,
        set_constructor_parameters
    );
}

#[derive(Debug, Clone, TransactionProto)]
#[hedera_derive(proto(
    proto_enum = "ContractCreateInstance",
    proto_type = "ContractCreateTransactionBody"
))]
struct Proto {
    #[hedera_derive(to_option_proto)]
    pub file_id: Option<FileId>,
    #[hedera_derive(to_option_proto)]
    pub admin_key: Option<Key>,
    pub gas: i64,
    pub initial_balance: i64,
    #[hedera_derive(to_option_proto)]
    pub proxy_account_id: Option<AccountId>,
    #[hedera_derive(to_option_proto)]
    pub auto_renew_period: Option<Duration>,
    pub constructor_parameters: Vec<u8>,
    #[hedera_derive(to_option_proto)]
    pub shard_id: Option<ShardId>,
    #[hedera_derive(to_option_proto)]
    pub realm_id: Option<RealmId>,
    #[hedera_derive(to_option_proto)]
    pub new_realm_admin_key: Option<Key>,
    pub memo: String,
}

impl Proto {
    pub fn new() -> Self {
        Proto {
            file_id: None,
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
        }
    }
}
