use crate::entity_id::validate_option_id_checksum;

use crate::transaction::Transaction;
use crate::AccountId;
use crate::Client;
use crate::Hbar;
use crate::HederaError;
use crate::TokenId;
use hedera_rust_client_derive::{TransactionExecute, TransactionProto, TransactionSchedule};

#[derive(TransactionSchedule, TransactionExecute, Debug, Clone, PartialEq)]
#[hedera_rust_client_derive(service(
    method_service_name = "token",
    method_service_fn = "grant_kyc_to_token_account"
))]
pub struct TokenGrantKycTransaction {
    transaction: Transaction,
    services: Proto,
}

impl TokenGrantKycTransaction {
    pub fn new() -> TokenGrantKycTransaction {
        let transaction = Transaction::with_max_transaction_fee(Hbar::new(30.0));
        let services = Proto::new();
        TokenGrantKycTransaction {
            transaction,
            services,
        }
    }

    fn validate_network_on_ids(&self, client: &Client) -> Result<(), HederaError> {
        validate_option_id_checksum(&self.services.token, client)?;
        validate_option_id_checksum(&self.services.account, client)?;
        Ok(())
    }

    // token
    gen_transaction_token_fns!();

    // account_id
    gen_transaction_account_fns!();
}

#[derive(Debug, Clone, PartialEq, TransactionProto)]
#[hedera_rust_client_derive(proto(
    proto_enum = "TokenGrantKyc",
    proto_type = "TokenGrantKycTransactionBody"
))]
struct Proto {
    #[hedera_rust_client_derive(to_option_proto)]
    pub token: Option<TokenId>,
    #[hedera_rust_client_derive(to_option_proto)]
    pub account: Option<AccountId>,
}

impl Proto {
    pub fn new() -> Self {
        Proto {
            token: None,
            account: None,
        }
    }
}
