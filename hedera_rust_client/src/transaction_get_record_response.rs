use std::convert::TryFrom;

use crate::error::HederaError;
use crate::proto::services::{self};
use crate::response_header::ResponseHeader;
use crate::transaction_record::TransactionRecord;

#[derive(Clone, Debug, PartialEq)]
pub struct TransactionGetRecordResponse {
    pub header: ResponseHeader,
    pub transaction_record: Option<TransactionRecord>,
    pub duplicate_transaction_records: Vec<TransactionRecord>,
    pub child_transaction_records: Vec<TransactionRecord>,
}

impl TryFrom<services::TransactionGetRecordResponse> for TransactionGetRecordResponse {
    type Error = HederaError;
    fn try_from(
        services: services::TransactionGetRecordResponse,
    ) -> Result<TransactionGetRecordResponse, Self::Error> {
        let header = services.header.ok_or(HederaError::NoResponseHeader)?;
        Ok(TransactionGetRecordResponse {
            header: ResponseHeader::try_from(header)?,
            transaction_record: services
                .transaction_record
                .map(TransactionRecord::try_from)
                .transpose()?,
            duplicate_transaction_records: services
                .duplicate_transaction_records
                .into_iter()
                .map(TransactionRecord::try_from)
                .collect::<Result<Vec<TransactionRecord>, HederaError>>()?,
            child_transaction_records: services
                .child_transaction_records
                .into_iter()
                .map(TransactionRecord::try_from)
                .collect::<Result<Vec<TransactionRecord>, HederaError>>()?,
        })
    }
}
