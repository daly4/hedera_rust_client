use chrono::Duration;
use hedera_rust_client_derive::{TransactionExecute, TransactionProto, TransactionSchedule};
use std::convert::TryFrom;

use crate::account_id::AccountId;
use crate::client::Client;
use crate::entity_id::validate_option_id_checksum;
use crate::key::Key;
use crate::transaction::Transaction;
use crate::utils::DEFAULT_DURATION;
use crate::Hbar;
use crate::HederaError;
use crate::RealmId;
use crate::ShardId;
use crate::StakedId;

#[derive(TransactionSchedule, TransactionExecute, Debug, Clone, PartialEq)]
#[hedera_rust_client_derive(service(
    method_service_name = "crypto",
    method_service_fn = "create_account"
))]
pub struct AccountCreateTransaction {
    transaction: Transaction,
    services: Proto,
}

impl AccountCreateTransaction {
    pub fn new() -> AccountCreateTransaction {
        let transaction = Transaction::with_max_transaction_fee(Hbar::new(5.0));
        let mut services = Proto::new();
        services.auto_renew_period = Some(*DEFAULT_DURATION);
        AccountCreateTransaction {
            transaction,
            services,
        }
    }

    fn validate_network_on_ids(&self, client: &Client) -> Result<(), HederaError> {
        validate_option_id_checksum(&self.services.proxy_account_id, client)?;
        Ok(())
    }

    gen_transaction_key_fns!();
    gen_transaction_initial_balance_u64!();
    gen_transaction_auto_renew_period_fns!();
    gen_transaction_receiver_sig_required_fns!();
    gen_transaction_memo_fns!();
    gen_transaction_max_automatic_token_associations_fns!();
    gen_transaction_decline_award_fns!();
    gen_transaction_staked_id_option_fns!();
}

#[derive(Debug, Clone, PartialEq, TransactionProto)]
#[hedera_rust_client_derive(proto(
    proto_enum = "CryptoCreateAccount",
    proto_type = "CryptoCreateTransactionBody"
))]
struct Proto {
    #[hedera_rust_client_derive(to_option_proto)]
    pub key: Option<Key>,
    pub initial_balance: u64,
    #[hedera_rust_client_derive(to_option_proto)]
    pub proxy_account_id: Option<AccountId>,
    pub send_record_threshold: u64,    // depreciated
    pub receive_record_threshold: u64, // depreciated
    pub receiver_sig_required: bool,
    #[hedera_rust_client_derive(to_option_proto)]
    pub auto_renew_period: Option<Duration>,
    #[hedera_rust_client_derive(to_option_proto)]
    pub shard_id: Option<ShardId>,
    #[hedera_rust_client_derive(to_option_proto)]
    pub realm_id: Option<RealmId>,
    #[hedera_rust_client_derive(to_option_proto)]
    pub new_realm_admin_key: Option<Key>,
    pub memo: String,
    pub max_automatic_token_associations: i32,
    pub decline_reward: bool,
    #[hedera_rust_client_derive(to_option_proto)]
    pub staked_id: Option<StakedId>,
}

impl Proto {
    pub fn new() -> Self {
        Proto {
            key: None,
            initial_balance: 0,
            proxy_account_id: None,
            send_record_threshold: 0,
            receive_record_threshold: 0,
            receiver_sig_required: false,
            auto_renew_period: None,
            shard_id: None,
            realm_id: None,
            new_realm_admin_key: None,
            memo: "".to_string(),
            max_automatic_token_associations: 0,
            decline_reward: false,
            staked_id: None,
        }
    }
}
