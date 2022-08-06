use hedera_derive::{TransactionExecute, TransactionProto};
use std::collections::HashMap;

use crate::entity_id::validate_id_checksum;
use crate::proto::{services, ToProto};
use crate::token_nft_transfer::TokenNftTransfer;
use crate::token_transfer::TokenTransfer;
use crate::transaction::Transaction;
use crate::AccountId;
use crate::Client;
use crate::Hbar;
use crate::HederaError;
use crate::NftId;
use crate::TokenId;

#[derive(TransactionExecute, Debug, Clone)]
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
            match tx {
                TokenTransferList::Token(tx) => {
                    for t in tx.transfers.values() {
                        validate_id_checksum(&t.account_id, client)?;
                    }
                }
                TokenTransferList::Nft(nfts) => {
                    for t in nfts.values() {
                        validate_id_checksum(&t.sender_account_id, client)?;
                        validate_id_checksum(&t.receiver_account_id, client)?;
                    }
                }
            }
        }
        Ok(())
    }

    pub fn token_transfers(&self) -> Result<HashMap<TokenId, Vec<TokenTransfer>>, HederaError> {
        let mut map = HashMap::new();

        if self.services.token_transfers.is_empty() {
            return Ok(map);
        }

        for (token_id, token_transfers) in self.services.token_transfers.iter() {
            if let TokenTransferList::Token(tx) = token_transfers {
                map.insert(token_id.clone(), tx.transfers.values().cloned().collect());
            }
        }
        Ok(map)
    }

    pub fn nft_transfers(&self) -> Result<HashMap<TokenId, Vec<TokenNftTransfer>>, HederaError> {
        let mut map = HashMap::new();

        if self.services.token_transfers.is_empty() {
            return Ok(map);
        }

        for (token_id, token_transfers) in self.services.token_transfers.iter() {
            if let TokenTransferList::Nft(nfts) = token_transfers {
                map.insert(token_id.clone(), nfts.values().cloned().collect());
            }
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
                if let TokenTransferList::Token(tx) = token_transfer {
                    tx.transfers.insert(account_id, amount_transfer);
                }
            }
            None => {
                let mut transfers = HashMap::new();
                transfers.insert(account_id, amount_transfer);
                let tx = TokenTransferList::Token(TokenTransfers {
                    transfers,
                    expected_decimals: decimals,
                });
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
                if let TokenTransferList::Nft(nfts) = token_transfer {
                    nfts.insert(nft_id.serial_number, nft_transfer);
                }
            }
            None => {
                let mut nfts = HashMap::new();
                nfts.insert(nft_id.serial_number, nft_transfer);
                let tx = TokenTransferList::Nft(nfts);
                self.services.token_transfers.insert(nft_id.token_id, tx);
            }
        }

        Ok(self)
    }
}

#[derive(Debug, Clone, TransactionProto)]
#[hedera_derive(proto(
    proto_enum = "CryptoTransfer",
    proto_type = "CryptoTransferTransactionBody"
))]
struct Proto {
    #[hedera_derive(to_proto_with_fn = "to_transfers")]
    pub transfers: Option<HashMap<AccountId, TokenTransfer>>,
    #[hedera_derive(to_proto_with_fn = "to_token_transfers")]
    pub token_transfers: HashMap<TokenId, TokenTransferList>,
}

impl Proto {
    pub fn new() -> Self {
        Proto {
            transfers: None,
            token_transfers: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone)]
enum TokenTransferList {
    Token(TokenTransfers),
    Nft(HashMap<i64, TokenNftTransfer>),
}

#[derive(Debug, Clone)]
pub struct TokenTransfers {
    pub transfers: HashMap<AccountId, TokenTransfer>,
    pub expected_decimals: Option<u32>,
}

fn to_transfers(
    tx: &Option<HashMap<AccountId, TokenTransfer>>,
) -> Result<Option<services::TransferList>, HederaError> {
    match tx {
        Some(tx) => {
            let mut account_amounts = Vec::with_capacity(tx.len());
            for v in tx.values() {
                account_amounts.push(v.to_proto()?);
            }
            Ok(Some(services::TransferList { account_amounts }))
        }
        None => Ok(None),
    }
}

fn to_token_transfers(
    tx: &HashMap<TokenId, TokenTransferList>,
) -> Result<Vec<services::TokenTransferList>, HederaError> {
    let mut list = Vec::with_capacity(tx.len());
    for (k, v) in tx.iter() {
        let mut services = services::TokenTransferList {
            token: Some(k.to_proto()?),
            transfers: Vec::new(),
            nft_transfers: Vec::new(),
            expected_decimals: None,
        };

        match v {
            TokenTransferList::Token(tx) => {
                let mut transfers = Vec::with_capacity(tx.transfers.len());
                for v_t in tx.transfers.values() {
                    transfers.push(v_t.to_proto()?);
                }
                services.transfers = transfers;
                services.expected_decimals = tx.expected_decimals;
            }
            TokenTransferList::Nft(nfts) => {
                let mut nft_transfers = Vec::with_capacity(nfts.len());
                for v_t in nfts.values() {
                    nft_transfers.push(v_t.to_proto()?);
                }
                services.nft_transfers = nft_transfers;
            }
        }
        list.push(services);
    }
    Ok(list)
}
