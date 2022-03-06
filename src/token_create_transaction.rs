use chrono::{DateTime, Utc, Duration};
use hedera_derive::{TransactionExecute, TransactionProto, TransactionSchedule};

use crate::entity_id::{validate_option_id_checksum, ValidateChecksum};
use crate::transaction::Transaction;
use crate::AccountId;
use crate::Client;
use crate::CustomFee;
use crate::Hbar;
use crate::HederaError;
use crate::Key;
use crate::TokenSupplyType;
use crate::TokenType;
use crate::utils::DEFAULT_DURATION;

#[derive(TransactionSchedule, TransactionExecute, Debug, Clone)]
#[hedera_derive(service(method_service_name = "token", method_service_fn = "create_token"))]
pub struct TokenCreateTransaction {
    transaction: Transaction,
    services: Proto,
}

impl TokenCreateTransaction {
    pub fn new() -> TokenCreateTransaction {
        let transaction = Transaction::with_max_transaction_fee(Hbar::new(30.0));
        let mut services = Proto::new();
        services.auto_renew_period = Some(*DEFAULT_DURATION); //min
        services.expiry = Some(Utc::now() + *DEFAULT_DURATION);
        TokenCreateTransaction { transaction, services }
    }

    fn validate_network_on_ids(&self, client: &Client) -> Result<(), HederaError> {
        validate_option_id_checksum(&self.services.treasury, client)?;
        validate_option_id_checksum(&self.services.auto_renew_account, client)?;
        for fee in self.services.custom_fees.iter() {
            fee.validate_checksum(client)?;
        }
        Ok(())
    }

    // name
    gen_transaction_name_fns!();

    // symbol
    gen_transaction_symbol_fns!();

    // decimals
    gen_transaction_decimals_fns!();

    // initial_supply
    gen_transaction_initial_supply_fns!();

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

    // freeze_default
    gen_transaction_freeze_default_fns!();

    // expiry -> expiration_time/set_expiration_time
    gen_transaction_expiry_time_fns!();

    // auto_renew_account
    gen_transaction_auto_renew_account_fns!();

    // auto_renew_period
    gen_transaction_auto_renew_period_fns!();

    // memo
    gen_transaction_memo_fns!();

    // token_type
    gen_transaction_token_type_fns!();

    // supply_type
    gen_transaction_supply_type_fns!();

    // max_supply
    gen_transaction_max_supply_fns!();

    // fee_schedule_key
    gen_transaction_fee_schedule_key_fns!();

    // custom_fees
    gen_transaction_custom_fees_fns!();

    // pause_key
    gen_transaction_pause_key_fns!();
}

#[derive(Debug, Clone, TransactionProto)]
#[hedera_derive(proto(
    proto_enum = "TokenCreation",
    proto_type = "TokenCreateTransactionBody"
))]
struct Proto {
    pub name: String,
    pub symbol: String,
    pub decimals: u32,
    pub initial_supply: u64,
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
    pub freeze_default: bool,
    #[hedera_derive(to_option_proto)]
    pub expiry: Option<DateTime<Utc>>,
    #[hedera_derive(to_option_proto)]
    pub auto_renew_account: Option<AccountId>,
    #[hedera_derive(to_option_proto)]
    pub auto_renew_period: Option<Duration>,
    pub memo: String,
    #[hedera_derive(to_proto)]
    pub token_type: TokenType,
    #[hedera_derive(to_proto)]
    pub supply_type: TokenSupplyType,
    pub max_supply: i64,
    #[hedera_derive(to_option_proto)]
    pub fee_schedule_key: Option<Key>,
    #[hedera_derive(to_proto_vec = "CustomFee")]
    pub custom_fees: Vec<CustomFee>,
    #[hedera_derive(to_option_proto)]
    pub pause_key: Option<Key>,
}

impl Proto {
    pub fn new() -> Self {
        Proto {
            name: String::new(),
            symbol: String::new(),
            decimals: 0,
            initial_supply: 0,
            treasury: None,
            admin_key: None,
            kyc_key: None,
            freeze_key: None,
            wipe_key: None,
            supply_key: None,
            freeze_default: false,
            expiry: None,
            auto_renew_account: None,
            auto_renew_period: None,
            memo: String::new(),
            token_type: TokenType::FungibleCommon,
            supply_type: TokenSupplyType::Infinite,
            max_supply: 0,
            fee_schedule_key: None,
            custom_fees: Vec::new(),
            pause_key: None,
        }
    }
}
