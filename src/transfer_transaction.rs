use hedera_derive::{TransactionExecute, TransactionProto};
use std::collections::HashMap;

use crate::entity_id::validate_id_checksum;
use crate::proto::{services, ToProto};
use crate::token_nft_transfer::TokenNftTransfer;
use crate::token_transfer::TokenTransfer;
use crate::token_transfer_list::{
    token_transfer_list_hashmap_to_proto_vec, AccountIdTokenTransferHashMap,
    TokenIdTokenTransferListHashMap, TokenTransferList,
};
use crate::transaction::Transaction;
use crate::AccountId;
use crate::Client;
use crate::Hbar;
use crate::HederaError;
use crate::NftId;
use crate::TokenId;

#[derive(TransactionExecute, Debug, Clone, PartialEq)]
#[hedera_derive(service(method_service_name = "crypto", method_service_fn = "crypto_transfer"))]
pub struct TransferTransaction {
    transaction: Transaction,
    services: Proto,
}

impl TransferTransaction {
    pub fn new() -> TransferTransaction {
        let transaction = Transaction::with_max_transaction_fee(Hbar::new(1.0));
        let services = Proto::new();
        TransferTransaction {
            transaction,
            services,
        }
    }

    fn validate_network_on_ids(&self, client: &Client) -> Result<(), HederaError> {
        if let Some(transfers) = &self.services.transfers {
            for tx in transfers.values() {
                validate_id_checksum(&tx.account_id, client)?;
            }
        }
        for (token_id, tx) in self.services.token_transfers.iter() {
            validate_id_checksum(token_id, client)?;
            for t in tx.transfers.values() {
                validate_id_checksum(&t.account_id, client)?;
            }
            for t in tx.nft_transfers.values() {
                validate_id_checksum(&t.sender_account_id, client)?;
                validate_id_checksum(&t.receiver_account_id, client)?;
            }
        }
        Ok(())
    }

    pub fn token_transfers(&self) -> Result<HashMap<TokenId, Vec<TokenTransfer>>, HederaError> {
        let mut map = HashMap::new();

        if self.services.token_transfers.is_empty() {
            return Ok(map);
        }

        for (token_id, tx) in self.services.token_transfers.iter() {
            map.insert(*token_id, tx.transfers.values().cloned().collect());
        }
        Ok(map)
    }

    pub fn nft_transfers(&self) -> Result<HashMap<TokenId, Vec<TokenNftTransfer>>, HederaError> {
        let mut map = HashMap::new();

        if self.services.token_transfers.is_empty() {
            return Ok(map);
        }

        for (token_id, tx) in self.services.token_transfers.iter() {
            map.insert(*token_id, tx.nft_transfers.values().cloned().collect());
        }
        Ok(map)
    }

    pub fn hbar_transfers(&self) -> Result<HashMap<AccountId, Hbar>, HederaError> {
        let mut map = HashMap::new();
        if let Some(transfers) = &self.services.transfers {
            for (account_id, amt) in transfers.iter() {
                map.insert(*account_id, Hbar::from_tinybar(amt.amount));
            }
        }
        Ok(map)
    }

    pub fn add_hbar_transfer(
        &mut self,
        account_id: AccountId,
        amount: Hbar,
    ) -> Result<&mut Self, HederaError> {
        self.transaction.require_not_frozen()?;
        let token_transfer = TokenTransfer {
            account_id,
            amount: amount.as_tinybar(),
        };
        match &mut self.services.transfers {
            Some(v) => {
                v.insert(account_id, token_transfer);
            }
            None => {
                let mut amounts = HashMap::new();
                amounts.insert(account_id, token_transfer);
                self.services.transfers = Some(amounts);
            }
        }
        Ok(self)
    }

    pub fn add_token_transfer(
        &mut self,
        token_id: TokenId,
        account_id: AccountId,
        value: i64,
        decimals: Option<u32>,
    ) -> Result<&mut Self, HederaError> {
        self.transaction.require_not_frozen()?;
        let amount_transfer = TokenTransfer {
            account_id,
            amount: value,
        };

        match self.services.token_transfers.get_mut(&token_id) {
            Some(token_transfer) => {
                token_transfer.transfers.insert(account_id, amount_transfer);
            }
            None => {
                let mut transfers = HashMap::new();
                transfers.insert(account_id, amount_transfer);
                let tx = TokenTransferList {
                    token_id,
                    transfers,
                    expected_decimals: decimals,
                    nft_transfers: HashMap::new(),
                };
                self.services.token_transfers.insert(token_id, tx);
            }
        }
        Ok(self)
    }

    pub fn add_nft_transfer(
        &mut self,
        nft_id: NftId,
        sender_account_id: AccountId,
        receiver_account_id: AccountId,
    ) -> Result<&mut Self, HederaError> {
        self.transaction.require_not_frozen()?;
        let nft_transfer = TokenNftTransfer {
            sender_account_id,
            receiver_account_id,
            serial_number: nft_id.serial_number,
        };
        match self.services.token_transfers.get_mut(&nft_id.token_id) {
            Some(token_transfer) => {
                token_transfer
                    .nft_transfers
                    .insert(nft_id.serial_number, nft_transfer);
            }
            None => {
                let mut nft_transfers = HashMap::new();
                nft_transfers.insert(nft_id.serial_number, nft_transfer);
                let tx = TokenTransferList {
                    token_id: nft_id.token_id,
                    transfers: HashMap::new(),
                    expected_decimals: None,
                    nft_transfers,
                };
                self.services.token_transfers.insert(nft_id.token_id, tx);
            }
        }

        Ok(self)
    }
}

#[derive(Debug, Clone, PartialEq, TransactionProto)]
#[hedera_derive(proto(
    proto_enum = "CryptoTransfer",
    proto_type = "CryptoTransferTransactionBody"
))]
struct Proto {
    #[hedera_derive(to_proto_with_fn = "token_transfer_account_hashmap_to_proto")]
    pub transfers: Option<AccountIdTokenTransferHashMap>,
    #[hedera_derive(to_proto_with_fn = "token_transfer_list_hashmap_to_proto_vec")]
    pub token_transfers: TokenIdTokenTransferListHashMap,
}

impl Proto {
    pub fn new() -> Self {
        Proto {
            transfers: None,
            token_transfers: HashMap::new(),
        }
    }
}

pub fn token_transfer_account_hashmap_to_proto(
    tx: &Option<AccountIdTokenTransferHashMap>,
) -> Result<Option<services::TransferList>, HederaError> {
    match tx {
        Some(tx) => Ok(Some(services::TransferList {
            account_amounts: tx
                .values()
                .map(|v| v.to_proto())
                .collect::<Result<Vec<services::AccountAmount>, HederaError>>()?,
        })),
        None => Ok(None),
    }
}
