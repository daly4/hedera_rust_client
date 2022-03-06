use crate::entity_id::{validate_option_id_checksum, ValidateChecksum};
use crate::transaction::Transaction;
use crate::Client;
use crate::CustomFee;
use crate::Hbar;
use crate::HederaError;
use crate::TokenId;
use hedera_derive::{TransactionExecute, TransactionProto, TransactionSchedule};

#[derive(TransactionSchedule, TransactionExecute, Debug, Clone)]
#[hedera_derive(service(
    method_service_name = "token",
    method_service_fn = "update_token_fee_schedule"
))]
pub struct TokenFeeScheduleUpdateTransaction {
    transaction: Transaction,
    services: Proto,
}

impl TokenFeeScheduleUpdateTransaction {
    pub fn new() -> TokenFeeScheduleUpdateTransaction {
        let transaction = Transaction::with_max_transaction_fee(Hbar::new(5.0));
        let services = Proto::new();
        TokenFeeScheduleUpdateTransaction { transaction, services }
    }

    fn validate_network_on_ids(&self, client: &Client) -> Result<(), HederaError> {
        validate_option_id_checksum(&self.services.token_id, client)?;
        for fee in self.services.custom_fees.iter() {
            fee.validate_checksum(client)?;
        }
        Ok(())
    }

    // token
    gen_transaction_token_id_fns!();

    // custom_fees
    gen_transaction_custom_fees_fns!();
}

#[derive(Debug, Clone, TransactionProto)]
#[hedera_derive(proto(
    proto_enum = "TokenFeeScheduleUpdate",
    proto_type = "TokenFeeScheduleUpdateTransactionBody"
))]
struct Proto {
    #[hedera_derive(to_option_proto)]
    pub token_id: Option<TokenId>,
    #[hedera_derive(to_proto_vec = "CustomFee")]
    pub custom_fees: Vec<CustomFee>,
}

impl Proto {
    pub fn new() -> Self {
        Proto {
            token_id: None,
            custom_fees: Vec::new(),
        }
    }
}
