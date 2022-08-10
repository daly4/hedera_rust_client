use hedera_rust_client_derive::{QueryExecuteAsync, QueryGetCost};
use std::convert::TryFrom;

use crate::error::HederaError;
use crate::executor::Response;
use crate::proto::{services, ToProto};
use crate::query::Query;
use crate::query_header::QueryHeader;
use crate::status::Status;
use crate::TransactionReceipt;

fn transaction_receipt_query_should_retry(status: &Status, response: &Response) -> bool {
    let status = *status;
    if status == Status::Busy || status == Status::Unknown || status == Status::ReceiptNotFound {
        return true;
    }

    if status != Status::Ok {
        return false;
    }

    let mut status: Option<Status> = None;
    if let Ok(services::response::Response::TransactionGetReceipt(ref res)) =
        response.get_proto_query()
    {
        if let Some(receipt) = &res.receipt {
            status = Status::from_i32(receipt.status);
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

#[derive(QueryGetCost, QueryExecuteAsync, Debug, Clone, PartialEq)]
#[hedera_rust_client_derive(
    proto(
        proto_enum = "TransactionGetReceipt",
        response_enum = "TransactionGetReceipt",
    ),
    service(
        method_service_name = "crypto",
        method_service_fn = "get_transaction_receipts",
    ),
    exe(should_retry = "transaction_receipt_query_should_retry",)
)]
pub struct TransactionReceiptQuery {
    query: Query,
    header: QueryHeader,
    services: services::TransactionGetReceiptQuery,
}

impl TransactionReceiptQuery {
    pub fn new() -> TransactionReceiptQuery {
        let header = QueryHeader::new();
        let query = Query::new(false);
        let services = services::TransactionGetReceiptQuery {
            header: None,
            transaction_id: None,
            include_duplicates: false,
            include_child_receipts: false,
        };
        TransactionReceiptQuery {
            query,
            header,
            services,
        }
    }

    gen_query_transaction_id_fns!();

    gen_query_include_duplicates_fns!();

    gen_query_include_child_receipts_fns!();

    gen_query_execute!(
        TransactionReceipt,
        TransactionGetReceipt,
        (|res: services::TransactionGetReceiptResponse| {
            if let Some(receipt) = res.receipt {
                return TransactionReceipt::try_from(receipt);
            }
            Err(HederaError::MissingInProto("receipt".to_string()))
        })
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transaction_receipt_query_should_retry() {
        let status = Status::Unknown;

        let res = Response::Query(services::Response {
            response: Some(services::response::Response::TransactionGetReceipt(
                services::TransactionGetReceiptResponse {
                    header: Some(services::ResponseHeader {
                        node_transaction_precheck_code: 0,
                        response_type: 0,
                        cost: 0,
                        state_proof: [].to_vec(),
                    }),
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
                    duplicate_transaction_receipts: [].to_vec(),
                    child_transaction_receipts: [].to_vec(),
                },
            )),
        });

        let should_retry = transaction_receipt_query_should_retry(&status, &res);
        assert!(should_retry);

        let status = Status::Ok;
        let should_retry = transaction_receipt_query_should_retry(&status, &res);
        assert!(should_retry);
    }
}
