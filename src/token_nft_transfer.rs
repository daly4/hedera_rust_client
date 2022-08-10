use std::convert::TryFrom;

use crate::account_id::AccountId;
use crate::error::HederaError;
use crate::proto::{services::NftTransfer, ToProto};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TokenNftTransfer {
    pub sender_account_id: AccountId,
    pub receiver_account_id: AccountId,
    pub serial_number: i64,
}

impl TryFrom<NftTransfer> for TokenNftTransfer {
    type Error = HederaError;
    fn try_from(services: NftTransfer) -> Result<TokenNftTransfer, Self::Error> {
        Ok(TokenNftTransfer {
            sender_account_id: match services.sender_account_id {
                Some(account_id) => AccountId::try_from(account_id)?,
                None => return Err(HederaError::MissingInProto("sender_account_id".to_string())),
            },
            receiver_account_id: match services.receiver_account_id {
                Some(account_id) => AccountId::try_from(account_id)?,
                None => {
                    return Err(HederaError::MissingInProto(
                        "receiver_account_id".to_string(),
                    ))
                }
            },
            serial_number: services.serial_number,
        })
    }
}

impl ToProto<NftTransfer> for TokenNftTransfer {
    fn to_proto(&self) -> Result<NftTransfer, HederaError> {
        Ok(NftTransfer {
            sender_account_id: Some(self.sender_account_id.to_proto()?),
            receiver_account_id: Some(self.receiver_account_id.to_proto()?),
            serial_number: self.serial_number,
            is_approval: false,
        })
    }
}
