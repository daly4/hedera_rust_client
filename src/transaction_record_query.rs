use hedera_derive::QueryExecuteAsyncWithCostCheck;
use num_traits::FromPrimitive;
use std::convert::TryFrom;

use crate::error::HederaError;
use crate::executor::Response;
use crate::proto::{services, ToProto};
use crate::query::Query;
use crate::query_header::QueryHeader;
use crate::status::Status;
use crate::TransactionRecord;

fn transaction_record_query_should_retry(status: &Status, response: &Response) -> bool {
    let status = *status;
    if status == Status::Busy || status == Status::Unknown || status == Status::ReceiptNotFound {
        return true;
    }

    if status != Status::Ok {
        return false;
    }
    let mut status: Option<Status> = None;
    if let Ok(services::response::Response::TransactionGetRecord(ref res)) =
        response.get_proto_query()
    {
        if let Some(record) = &res.transaction_record {
            if let Some(receipt) = &record.receipt {
                status = Status::from_i32(receipt.status);
            }
        }
    }
    if let Some(s) = status {
        if s == Status::Busy
            || s == Status::Unknown
            || s == Status::Ok
            || s == Status::ReceiptNotFound
            || s == Status::RecordNotFound
        {
            return true;
        }
    }
    false
}

#[derive(QueryExecuteAsyncWithCostCheck, Debug, Clone)]
#[hedera_derive(
    proto(
        proto_enum = "TransactionGetRecord",
        response_enum = "TransactionGetRecord",
    ),
    service(
        method_service_name = "crypto",
        method_service_fn = "get_tx_record_by_tx_id",
    ),
    exe(should_retry = "transaction_record_query_should_retry",)
)]
pub struct TransactionRecordQuery {
    query: Query,
    header: QueryHeader,
    services: services::TransactionGetRecordQuery,
}

impl TransactionRecordQuery {
    pub fn new() -> TransactionRecordQuery {
        let header = QueryHeader::new();
        let query = Query::new(false);
        let services = services::TransactionGetRecordQuery {
            header: None,
            transaction_id: None,
            include_duplicates: false,
            include_child_records: false,
        };
        TransactionRecordQuery {
            query,
            header,
            services,
        }
    }

    gen_query_transaction_id_fns!();

    gen_query_include_duplicates_fns!();

    gen_query_include_child_records_fns!();

    gen_query_execute_with_cost_check!(
        TransactionRecord,
        TransactionGetRecord,
        (|res: services::TransactionGetRecordResponse| {
            if let Some(record) = res.transaction_record {
                return TransactionRecord::try_from(record);
            }
            Err(HederaError::MissingInProto(
                "transaction_record".to_string(),
            ))
        })
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transaction_record_query_should_retry() {
        let status = Status::Unknown;

        let res = Response::Query(services::Response {
            response: Some(services::response::Response::TransactionGetRecord(
                services::TransactionGetRecordResponse {
                    header: Some(services::ResponseHeader {
                        node_transaction_precheck_code: 0,
                        response_type: 0,
                        cost: 0,
                        state_proof: [].to_vec(),
                    }),
                    transaction_record: Some(services::TransactionRecord {
                        receipt: Some(services::TransactionReceipt {
                            status: 21,
                            account_id: None,
                            file_id: None,
                            contract_id: None,
                            exchange_rate: None,
                            topic_id: None,
                            topic_sequence_number: 0,
                            topic_running_hash: [].to_vec(),
                            topic_running_hash_version: 0,
                            token_id: None,
                            new_total_supply: 0,
                            schedule_id: None,
                            scheduled_transaction_id: None,
                            serial_numbers: [].to_vec(),
                        }),
                        transaction_hash: [].to_vec(),
                        consensus_timestamp: None,
                        transaction_id: None,
                        memo: "".to_string(),
                        transaction_fee: 1,
                        transfer_list: None,
                        token_transfer_lists: [].to_vec(),
                        schedule_ref: None,
                        assessed_custom_fees: [].to_vec(),
                        automatic_token_associations: [].to_vec(),
                        parent_consensus_timestamp: None,
                        alias: [].to_vec(),
                        crypto_adjustments: [].to_vec(),
                        nft_adjustments: [].to_vec(),
                        token_adjustments: [].to_vec(),
                        body: None,
                    }),
                    duplicate_transaction_records: [].to_vec(),
                    child_transaction_records: [].to_vec(),
                },
            )),
        });

        let should_retry = transaction_record_query_should_retry(&status, &res);
        assert!(should_retry);

        let status = Status::Ok;
        let should_retry = transaction_record_query_should_retry(&status, &res);
        assert!(should_retry);
    }
}
