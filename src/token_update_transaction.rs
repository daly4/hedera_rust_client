use crate::entity_id::validate_option_id_checksum;

use crate::transaction::Transaction;
use crate::AccountId;
use crate::Client;
use crate::Hbar;
use crate::HederaError;
use crate::Key;
use crate::TokenId;
use chrono::{DateTime, Duration, Utc};
use hedera_derive::{TransactionExecute, TransactionProto, TransactionSchedule};

#[derive(TransactionSchedule, TransactionExecute, Debug, Clone)]
#[hedera_derive(service(method_service_name = "token", method_service_fn = "update_token"))]
pub struct TokenUpdateTransaction {
    transaction: Transaction,
    services: Proto,
}

impl TokenUpdateTransaction {
    pub fn new() -> TokenUpdateTransaction {
        let transaction = Transaction::with_max_transaction_fee(Hbar::new(30.0));
        let services = Proto::new();
        TokenUpdateTransaction {
            transaction,
            services,
        }
    }

    fn validate_network_on_ids(&self, client: &Client) -> Result<(), HederaError> {
        validate_option_id_checksum(&self.services.token, client)?;
        validate_option_id_checksum(&self.services.treasury, client)?;
        validate_option_id_checksum(&self.services.auto_renew_account, client)?;
        Ok(())
    }

    // token
    gen_transaction_token_fns!();

    // name
    gen_transaction_name_fns!();

    // symbol
    gen_transaction_symbol_fns!();

    // treasury
    gen_transaction_treasury_fns!();

    // admin_key
    gen_transaction_admin_key_fns!();

    // kyc_key
    gen_transaction_kyc_key_fns!();

    // freeze_key
    gen_transaction_freeze_key_fns!();

    // wipe_key
    gen_transaction_wipe_key_fns!();

    // supply_key
    gen_transaction_supply_key_fns!();

    // expiry -> expiration_time/set_expiration_time
    gen_transaction_expiry_time_fns!();

    // auto_renew_account
    gen_transaction_auto_renew_account_fns!();

    // auto_renew_period
    gen_transaction_auto_renew_period_fns!();

    // memo
    gen_transaction_optional_memo_fns!();

    // fee_schedule_key
    gen_transaction_fee_schedule_key_fns!();

    // pause_key
    gen_transaction_pause_key_fns!();
}

#[derive(Debug, Clone, TransactionProto)]
#[hedera_derive(proto(proto_enum = "TokenUpdate", proto_type = "TokenUpdateTransactionBody"))]
struct Proto {
    #[hedera_derive(to_option_proto)]
    pub token: Option<TokenId>,
    pub name: String,
    pub symbol: String,
    #[hedera_derive(to_option_proto)]
    pub treasury: Option<AccountId>,
    #[hedera_derive(to_option_proto)]
    pub admin_key: Option<Key>,
    #[hedera_derive(to_option_proto)]
    pub kyc_key: Option<Key>,
    #[hedera_derive(to_option_proto)]
    pub freeze_key: Option<Key>,
    #[hedera_derive(to_option_proto)]
    pub wipe_key: Option<Key>,
    #[hedera_derive(to_option_proto)]
    pub supply_key: Option<Key>,
    #[hedera_derive(to_option_proto)]
    pub expiry: Option<DateTime<Utc>>,
    #[hedera_derive(to_option_proto)]
    pub auto_renew_account: Option<AccountId>,
    #[hedera_derive(to_option_proto)]
    pub auto_renew_period: Option<Duration>,
    pub memo: Option<String>,
    #[hedera_derive(to_option_proto)]
    pub fee_schedule_key: Option<Key>,
    #[hedera_derive(to_option_proto)]
    pub pause_key: Option<Key>,
}

impl Proto {
    pub fn new() -> Self {
        Proto {
            token: None,
            name: String::new(),
            symbol: String::new(),
            treasury: None,
            admin_key: None,
            kyc_key: None,
            freeze_key: None,
            wipe_key: None,
            supply_key: None,
            expiry: None,
            auto_renew_account: None,
            auto_renew_period: None,
            memo: None,
            fee_schedule_key: None,
            pause_key: None,
        }
    }
}
