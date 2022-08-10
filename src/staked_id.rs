use serde::{Deserialize, Serialize};

use crate::account_id::AccountId;
use crate::error::HederaError;
use crate::proto::services::{self};
use crate::proto::ToProto;

#[derive(Debug, Clone, Copy, PartialEq, Hash, Serialize, Deserialize)]
pub enum StakedId {
    StakedAccountId(AccountId),
    StakedNodeId(i64),
}

impl ToProto<services::staking_info::StakedId> for StakedId {
    fn to_proto(&self) -> Result<services::staking_info::StakedId, HederaError> {
        let pb = match &self {
            StakedId::StakedAccountId(id) => {
                services::staking_info::StakedId::StakedAccountId(id.to_proto()?)
            }
            StakedId::StakedNodeId(id) => services::staking_info::StakedId::StakedNodeId(*id),
        };
        Ok(pb)
    }
}

impl ToProto<services::contract_create_transaction_body::StakedId> for StakedId {
    fn to_proto(
        &self,
    ) -> Result<services::contract_create_transaction_body::StakedId, HederaError> {
        let pb = match &self {
            StakedId::StakedAccountId(id) => {
                services::contract_create_transaction_body::StakedId::StakedAccountId(
                    id.to_proto()?,
                )
            }
            StakedId::StakedNodeId(id) => {
                services::contract_create_transaction_body::StakedId::StakedNodeId(*id)
            }
        };
        Ok(pb)
    }
}

impl ToProto<services::contract_update_transaction_body::StakedId> for StakedId {
    fn to_proto(
        &self,
    ) -> Result<services::contract_update_transaction_body::StakedId, HederaError> {
        let pb = match &self {
            StakedId::StakedAccountId(id) => {
                services::contract_update_transaction_body::StakedId::StakedAccountId(
                    id.to_proto()?,
                )
            }
            StakedId::StakedNodeId(id) => {
                services::contract_update_transaction_body::StakedId::StakedNodeId(*id)
            }
        };
        Ok(pb)
    }
}

impl ToProto<services::crypto_create_transaction_body::StakedId> for StakedId {
    fn to_proto(&self) -> Result<services::crypto_create_transaction_body::StakedId, HederaError> {
        let pb = match &self {
            StakedId::StakedAccountId(id) => {
                services::crypto_create_transaction_body::StakedId::StakedAccountId(id.to_proto()?)
            }
            StakedId::StakedNodeId(id) => {
                services::crypto_create_transaction_body::StakedId::StakedNodeId(*id)
            }
        };
        Ok(pb)
    }
}

impl ToProto<services::crypto_update_transaction_body::StakedId> for StakedId {
    fn to_proto(&self) -> Result<services::crypto_update_transaction_body::StakedId, HederaError> {
        let pb = match &self {
            StakedId::StakedAccountId(id) => {
                services::crypto_update_transaction_body::StakedId::StakedAccountId(id.to_proto()?)
            }
            StakedId::StakedNodeId(id) => {
                services::crypto_update_transaction_body::StakedId::StakedNodeId(*id)
            }
        };
        Ok(pb)
    }
}
