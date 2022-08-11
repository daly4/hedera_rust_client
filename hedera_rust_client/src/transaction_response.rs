use std::convert::{TryFrom, TryInto};

use crate::error::HederaError;
use crate::proto::services::{self};
use crate::status::Status;
use crate::transaction_id::TransactionId;
use crate::AccountId;
use crate::Client;
use crate::Hbar;
use crate::TransactionGetRecordResponse;
use crate::TransactionReceipt;
use crate::TransactionReceiptQuery;
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
                return Err(HederaError::UnknownHederaStatusCode(
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

#[derive(Debug, Clone, PartialEq)]
pub struct TransactionResponse {
    pub transaction_id: Option<TransactionId>,
    pub scheduled_transaction_id: Option<TransactionId>,
    pub node_id: AccountId,
    pub hash: Vec<u8>,
}

impl TransactionResponse {
    pub async fn get_receipt(&self, client: &Client) -> Result<TransactionReceipt, HederaError> {
        match &self.transaction_id {
            Some(tx_id) => {
                let receipt = TransactionReceiptQuery::new()
                    .set_transaction_id(tx_id.clone())?
                    .set_node_account_ids(vec![self.node_id])?
                    .execute(client)
                    .await?;

                if receipt.status != Status::Success {
                    let status = receipt.status;
                    return Err(HederaError::ReceiptStatusError {
                        transaction_receipt: receipt,
                        status,
                        transaction_id: tx_id.clone(),
                    });
                }

                Ok(receipt)
            }
            None => Err(HederaError::ValueNotSet("transaction_id".to_string())),
        }
    }

    pub async fn get_record(
        &self,
        client: &Client,
    ) -> Result<TransactionGetRecordResponse, HederaError> {
        match &self.transaction_id {
            Some(tx_id) => {
                self.get_receipt(client).await?;
                let node_account_ids = vec![self.node_id];
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
