use chrono::{DateTime, Duration, Utc};
use hedera_derive::{TransactionExecute, TransactionProto, TransactionSchedule};

use crate::entity_id::validate_option_id_checksum;
use crate::proto::services;
use crate::transaction::Transaction;
use crate::AccountId;
use crate::Client;
use crate::Hbar;
use crate::HederaError;
use crate::Key;
use crate::StakedId;

#[derive(TransactionSchedule, TransactionExecute, Debug, Clone, PartialEq)]
#[hedera_derive(service(method_service_name = "crypto", method_service_fn = "update_account"))]
pub struct AccountUpdateTransaction {
    transaction: Transaction,
    services: Proto,
}

impl AccountUpdateTransaction {
    pub fn new() -> AccountUpdateTransaction {
        let transaction = Transaction::with_max_transaction_fee(Hbar::new(2.0));
        let services = Proto::new();
        AccountUpdateTransaction {
            transaction,
            services,
        }
    }
    fn validate_network_on_ids(&self, client: &Client) -> Result<(), HederaError> {
        validate_option_id_checksum(&self.services.account_id_to_update, client)?;
        validate_option_id_checksum(&self.services.proxy_account_id, client)?;
        Ok(())
    }
    gen_transaction_account_id_to_update_fns!();
    gen_transaction_key_fns!();
    gen_transaction_auto_renew_period_fns!();
    gen_transaction_expiration_time_fns!();
    gen_transaction_optional_memo_fns!();
    gen_transaction_max_automatic_token_associations_option_fns!();
    gen_transaction_decline_award_option_fns!();
    gen_transaction_staked_id_option_fns!();
}

#[derive(Debug, Clone, PartialEq, TransactionProto)]
#[hedera_derive(proto(
    proto_enum = "CryptoUpdateAccount",
    proto_type = "CryptoUpdateTransactionBody"
))]
struct Proto {
    #[hedera_derive(to_option_proto)]
    pub account_id_to_update: Option<AccountId>,
    #[hedera_derive(to_option_proto)]
    pub key: Option<Key>,
    #[hedera_derive(to_option_proto)]
    pub proxy_account_id: Option<AccountId>,
    pub proxy_fraction: i32,
    #[hedera_derive(to_option_proto)]
    pub auto_renew_period: Option<Duration>,
    #[hedera_derive(to_option_proto)]
    pub expiration_time: Option<DateTime<Utc>>,
    pub memo: Option<String>,
    pub max_automatic_token_associations: Option<i32>,
    pub decline_reward: Option<bool>,
    pub send_record_threshold_field:
        Option<services::crypto_update_transaction_body::SendRecordThresholdField>,
    pub receive_record_threshold_field:
        Option<services::crypto_update_transaction_body::ReceiveRecordThresholdField>,
    pub receiver_sig_required_field:
        Option<services::crypto_update_transaction_body::ReceiverSigRequiredField>,
    #[hedera_derive(to_option_proto)]
    pub staked_id: Option<StakedId>,
}

impl Proto {
    pub fn new() -> Self {
        Proto {
            account_id_to_update: None,
            key: None,
            proxy_account_id: None,
            proxy_fraction: 0,
            auto_renew_period: None,
            expiration_time: None,
            memo: None,
            max_automatic_token_associations: None,
            decline_reward: None,
            send_record_threshold_field: None,
            receive_record_threshold_field: None,
            receiver_sig_required_field: None,
            staked_id: None,
        }
    }
}
