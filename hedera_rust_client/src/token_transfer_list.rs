use std::collections::HashMap;
use std::convert::TryFrom;

use crate::proto::{services, ToProto};
use crate::token_nft_transfer::TokenNftTransfer;
use crate::token_transfer::TokenTransfer;
use crate::AccountId;
use crate::HederaError;
use crate::TokenId;

#[derive(Debug, Clone, PartialEq)]
pub struct TokenTransferList {
    pub token_id: TokenId,
    pub transfers: AccountIdTokenTransferHashMap,
    pub expected_decimals: Option<u32>,
    pub nft_transfers: SerialNumberTokenNftTransferHashMap,
}

impl TryFrom<services::TokenTransferList> for TokenTransferList {
    type Error = HederaError;
    fn try_from(services: services::TokenTransferList) -> Result<Self, Self::Error> {
        Ok(TokenTransferList {
            token_id: services
                .token
                .ok_or(HederaError::MissingInProto("token_id".to_string()))?
                .into(),
            transfers: proto_vec_account_amount_to_account_token_transfer_hashmap(
                services.transfers,
            )?,
            expected_decimals: services.expected_decimals,
            nft_transfers: proto_nft_transfer_vec_to_nft_transfer_hashmap(services.nft_transfers)?,
        })
    }
}

pub type SerialNumberTokenNftTransferHashMap = HashMap<i64, TokenNftTransfer>;

pub fn proto_nft_transfer_vec_to_nft_transfer_hashmap(
    nft_transfer_vec: Vec<services::NftTransfer>,
) -> Result<SerialNumberTokenNftTransferHashMap, HederaError> {
    let transfers = nft_transfer_vec
        .into_iter()
        .map(TokenNftTransfer::try_from)
        .collect::<Result<Vec<TokenNftTransfer>, HederaError>>()?
        .into_iter()
        .map(|tx| (tx.serial_number, tx))
        .collect::<HashMap<i64, TokenNftTransfer>>();
    Ok(transfers)
}

pub type TokenIdTokenTransferListHashMap = HashMap<TokenId, TokenTransferList>;

pub fn proto_token_transfer_list_vec_to_token_transfer_list_hashmap(
    token_transfer_list_vec: Vec<services::TokenTransferList>,
) -> Result<TokenIdTokenTransferListHashMap, HederaError> {
    let transfers = token_transfer_list_vec
        .into_iter()
        .map(TokenTransferList::try_from)
        .collect::<Result<Vec<TokenTransferList>, HederaError>>()?
        .into_iter()
        .map(|tx| (tx.token_id, tx))
        .collect::<HashMap<TokenId, TokenTransferList>>();
    Ok(transfers)
}

pub fn token_transfer_list_hashmap_to_proto_vec(
    map: &TokenIdTokenTransferListHashMap,
) -> Result<Vec<services::TokenTransferList>, HederaError> {
    let mut list = Vec::with_capacity(map.len());
    for (k, v) in map.iter() {
        list.push(services::TokenTransferList {
            token: Some(k.to_proto()?),
            transfers: v
                .transfers
                .values()
                .map(|x| x.to_proto())
                .collect::<Result<Vec<services::AccountAmount>, HederaError>>()?,
            nft_transfers: v
                .nft_transfers
                .values()
                .map(|x| x.to_proto())
                .collect::<Result<Vec<services::NftTransfer>, HederaError>>()?,
            expected_decimals: v.expected_decimals,
        });
    }
    Ok(list)
}

pub type AccountIdTokenTransferHashMap = HashMap<AccountId, TokenTransfer>;

pub fn proto_vec_account_amount_to_account_token_transfer_hashmap(
    services: Vec<services::AccountAmount>,
) -> Result<AccountIdTokenTransferHashMap, HederaError> {
    let transfers = services
        .into_iter()
        .map(TokenTransfer::try_from)
        .collect::<Result<Vec<TokenTransfer>, HederaError>>()?
        .into_iter()
        .map(|tx| (tx.account_id, tx))
        .collect::<HashMap<AccountId, TokenTransfer>>();
    Ok(transfers)
}

pub fn proto_transfer_list_to_account_token_transfer_hashmap(
    services: services::TransferList,
) -> Result<AccountIdTokenTransferHashMap, HederaError> {
    let transfers = services
        .account_amounts
        .into_iter()
        .map(TokenTransfer::try_from)
        .collect::<Result<Vec<TokenTransfer>, HederaError>>()?
        .into_iter()
        .map(|tx| (tx.account_id, tx))
        .collect::<HashMap<AccountId, TokenTransfer>>();
    Ok(transfers)
}
