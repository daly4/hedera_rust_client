use bytes::Bytes;
use chrono::Duration;
use prost::Message;
use std::convert::{TryFrom, TryInto};

use crate::error::HederaError;
use crate::proto::services::{
    self, schedulable_transaction_body::Data as SchTxData, transaction_body::Data as TxData,
};
use crate::proto::ToProto;
use crate::utils;
use crate::AccountId;
use crate::Hbar;
use crate::TransactionId;

#[derive(Debug, Clone, PartialEq)]
pub struct TransactionBody {
    pub transaction_id: Option<TransactionId>,
    pub node_account_id: Option<AccountId>,
    pub transaction_fee: Hbar,
    pub transaction_valid_duration: Option<Duration>,
    pub generate_record: bool, // Should a record of this transaction be generated? (A receipt is always generated, but the record is optional)
    pub memo: String,
    pub data: Option<TxData>,
}

impl TransactionBody {
    pub fn new() -> TransactionBody {
        TransactionBody {
            transaction_id: None,
            node_account_id: None,
            transaction_fee: Hbar::zero(),
            transaction_valid_duration: Some(Duration::seconds(120)),
            generate_record: false,
            memo: String::new(),
            data: None,
        }
    }

    pub fn to_proto_bytes(&self) -> Result<Vec<u8>, HederaError> {
        let body = self.to_proto()?;
        let mut buf = Vec::new();
        buf.reserve(body.encoded_len());
        match body.encode(&mut buf) {
            Ok(_) => Ok(buf),
            Err(e) => Err(HederaError::UnableToSerializeTransaction(e)),
        }
    }

    pub fn try_from_proto_bytes(bytes: Vec<u8>) -> Result<TransactionBody, HederaError> {
        match services::TransactionBody::decode(Bytes::from(bytes)) {
            Ok(proto) => Ok(TransactionBody::try_from(proto)?),
            Err(e) => Err(HederaError::UnableToDeserializeTransaction(e)),
        }
    }
}

impl ToProto<services::TransactionBody> for TransactionBody {
    fn to_proto(&self) -> Result<services::TransactionBody, HederaError> {
        let transaction_id = match &self.transaction_id {
            Some(v) => Some(v.to_proto()?),
            None => None,
        };
        let node_account_id = match &self.node_account_id {
            Some(v) => Some(v.to_proto()?),
            None => None,
        };
        let transaction_valid_duration = match &self.transaction_valid_duration {
            Some(v) => Some(v.to_proto()?),
            None => None,
        };
        #[allow(deprecated)]
        Ok(services::TransactionBody {
            transaction_id,
            node_account_id,
            transaction_fee: self.transaction_fee.as_tinybar_u64()?,
            transaction_valid_duration,
            generate_record: self.generate_record,
            memo: self.memo.clone(),
            data: self.data.clone(),
        })
    }
}

impl TryFrom<services::TransactionBody> for TransactionBody {
    type Error = HederaError;
    fn try_from(services: services::TransactionBody) -> Result<TransactionBody, Self::Error> {
        #[allow(deprecated)]
        Ok(TransactionBody {
            transaction_id: utils::optional_transaction_id(services.transaction_id)?,
            node_account_id: utils::optional_account_id(services.node_account_id)?,
            transaction_fee: services.transaction_fee.try_into()?,
            transaction_valid_duration: utils::optional_duration(
                services.transaction_valid_duration,
            )?,
            generate_record: services.generate_record, // Should a record of this transaction be generated? (A receipt is always generated, but the record is optional)
            memo: services.memo,
            data: services.data,
        })
    }
}

// Note: following tx body types are not supported b/c no scheduled version
// TxData::CryptoAddLiveHash(super::CryptoAddLiveHashTransactionBody),
// TxData::CryptoDeleteLiveHash(super::CryptoDeleteLiveHashTransactionBody),
// TxData::UncheckedSubmit(super::UncheckedSubmitBody),
// TxData::ScheduleCreate(super::ScheduleCreateTransactionBody),
// TxData::ScheduleSign(super::ScheduleSignTransactionBody),
impl TryFrom<TransactionBody> for services::SchedulableTransactionBody {
    type Error = HederaError;
    fn try_from(
        tx_body: TransactionBody,
    ) -> Result<services::SchedulableTransactionBody, Self::Error> {
        let data = match tx_body.data {
            Some(v) => {
                let sch_data = match v {
                    TxData::ContractCall(data) => SchTxData::ContractCall(data),
                    TxData::ContractCreateInstance(data) => SchTxData::ContractCreateInstance(data),
                    TxData::ContractUpdateInstance(data) => SchTxData::ContractUpdateInstance(data),
                    TxData::ContractDeleteInstance(data) => SchTxData::ContractDeleteInstance(data),
                    TxData::CryptoCreateAccount(data) => SchTxData::CryptoCreateAccount(data),
                    TxData::CryptoDelete(data) => SchTxData::CryptoDelete(data),
                    TxData::CryptoTransfer(data) => SchTxData::CryptoTransfer(data),
                    TxData::CryptoUpdateAccount(data) => SchTxData::CryptoUpdateAccount(data),
                    TxData::FileAppend(data) => SchTxData::FileAppend(data),
                    TxData::FileCreate(data) => SchTxData::FileCreate(data),
                    TxData::FileDelete(data) => SchTxData::FileDelete(data),
                    TxData::FileUpdate(data) => SchTxData::FileUpdate(data),
                    TxData::SystemDelete(data) => SchTxData::SystemDelete(data),
                    TxData::SystemUndelete(data) => SchTxData::SystemUndelete(data),
                    TxData::Freeze(data) => SchTxData::Freeze(data),
                    TxData::ConsensusCreateTopic(data) => SchTxData::ConsensusCreateTopic(data),
                    TxData::ConsensusUpdateTopic(data) => SchTxData::ConsensusUpdateTopic(data),
                    TxData::ConsensusDeleteTopic(data) => SchTxData::ConsensusDeleteTopic(data),
                    TxData::ConsensusSubmitMessage(data) => SchTxData::ConsensusSubmitMessage(data),
                    TxData::TokenCreation(data) => SchTxData::TokenCreation(data),
                    TxData::TokenFreeze(data) => SchTxData::TokenFreeze(data),
                    TxData::TokenUnfreeze(data) => SchTxData::TokenUnfreeze(data),
                    TxData::TokenGrantKyc(data) => SchTxData::TokenGrantKyc(data),
                    TxData::TokenRevokeKyc(data) => SchTxData::TokenRevokeKyc(data),
                    TxData::TokenDeletion(data) => SchTxData::TokenDeletion(data),
                    TxData::TokenUpdate(data) => SchTxData::TokenUpdate(data),
                    TxData::TokenMint(data) => SchTxData::TokenMint(data),
                    TxData::TokenBurn(data) => SchTxData::TokenBurn(data),
                    TxData::TokenWipe(data) => SchTxData::TokenWipe(data),
                    TxData::TokenAssociate(data) => SchTxData::TokenAssociate(data),
                    TxData::TokenDissociate(data) => SchTxData::TokenDissociate(data),
                    TxData::ScheduleDelete(data) => SchTxData::ScheduleDelete(data),
                    _ => return Err(HederaError::UnsupportedTransactionBodyType),
                };
                Some(sch_data)
            }
            None => None,
        };
        Ok(services::SchedulableTransactionBody {
            transaction_fee: tx_body.transaction_fee.as_tinybar_u64()?,
            memo: tx_body.memo,
            data,
        })
    }
}
