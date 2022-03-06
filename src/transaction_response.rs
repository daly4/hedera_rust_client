use num_traits::FromPrimitive;
use std::convert::{TryFrom, TryInto};

use crate::error::HederaError;
use crate::proto::services::{self};
use crate::status::Status;
use crate::transaction_id::TransactionId;
use crate::AccountId;
use crate::Client;
use crate::Hbar;
use crate::TransactionReceipt;
use crate::TransactionReceiptQuery;
use crate::TransactionRecord;
use crate::TransactionRecordQuery;

pub struct HederaResponse {
    pub status: Status,
    pub cost: Hbar,
}

impl TryFrom<services::TransactionResponse> for HederaResponse {
    type Error = HederaError;

    fn try_from(services: services::TransactionResponse) -> Result<HederaResponse, Self::Error> {
        let status = match Status::from_i32(services.node_transaction_precheck_code) {
            Some(v) => v,
            None => {
                return Err(HederaError::InvalidStatusCode(
                    services.node_transaction_precheck_code,
                ))
            }
        };
        Ok(HederaResponse {
            status,
            cost: services.cost.try_into()?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct TransactionResponse {
    pub transaction_id: Option<TransactionId>,
    pub scheduled_transaction_id: Option<TransactionId>,
    pub node_id: AccountId,
    pub hash: Vec<u8>,
}

impl TransactionResponse {
    pub async fn get_receipt(&self, client: &Client) -> Result<TransactionReceipt, HederaError> {
        match &self.transaction_id {
            Some(tx_id) => Ok(TransactionReceiptQuery::new()
                .set_transaction_id(tx_id.clone())?
                .set_node_account_ids(vec![self.node_id.clone()])?
                .execute(client)
                .await?),
            None => Err(HederaError::ValueNotSet("transaction_id".to_string())),
        }
    }

    pub async fn get_record(&self, client: &Client) -> Result<TransactionRecord, HederaError> {
        match &self.transaction_id {
            Some(tx_id) => {
                let node_account_ids = vec![self.node_id.clone()];
                TransactionReceiptQuery::new()
                    .set_transaction_id(tx_id.clone())?
                    .set_node_account_ids(node_account_ids.clone())?
                    .execute(client)
                    .await?;

                Ok(TransactionRecordQuery::new()
                    .set_transaction_id(tx_id.clone())?
                    .set_node_account_ids(node_account_ids)?
                    .execute(client)
                    .await?)
            }
            None => Err(HederaError::ValueNotSet("transaction_id".to_string())),
        }
    }
}
