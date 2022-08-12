use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

use crate::error::HederaError;
use crate::proto::services::NftId as ProtoNftId;
use crate::proto::ToProto;
use crate::token_id::TokenId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct NftId {
    pub token_id: TokenId,
    pub serial_number: i64,
}

impl NftId {
    pub fn new(token_id: TokenId, serial_number: i64) -> Self {
        NftId {
            token_id,
            serial_number,
        }
    }
}

impl TryFrom<ProtoNftId> for NftId {
    type Error = HederaError;
    fn try_from(services: ProtoNftId) -> Result<NftId, Self::Error> {
        let id = NftId {
            token_id: services
                .token_id
                .ok_or(HederaError::MissingInProto("token_id".to_string()))?
                .into(),
            serial_number: services.serial_number,
        };
        Ok(id)
    }
}

impl ToProto<ProtoNftId> for NftId {
    fn to_proto(&self) -> Result<ProtoNftId, HederaError> {
        Ok(ProtoNftId {
            token_id: Some(self.token_id.to_proto()?),
            serial_number: self.serial_number,
        })
    }
}
