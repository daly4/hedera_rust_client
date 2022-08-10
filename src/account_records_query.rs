use hedera_derive::QueryExecuteAsyncWithCostCheck;
use std::convert::TryFrom;

use crate::proto::{services, ToProto};
use crate::query::Query;
use crate::query_header::QueryHeader;
use crate::transaction_record::TransactionRecord;

#[derive(QueryExecuteAsyncWithCostCheck, Debug, Clone, PartialEq)]
#[hedera_derive(
    proto(
        proto_enum = "CryptoGetAccountRecords",
        response_enum = "CryptoGetAccountRecords",
    ),
    service(
        method_service_name = "crypto",
        method_service_fn = "get_account_records",
    )
)]
pub struct AccountRecordsQuery {
    query: Query,
    header: QueryHeader,
    services: services::CryptoGetAccountRecordsQuery,
}

impl AccountRecordsQuery {
    pub fn new() -> AccountRecordsQuery {
        let header = QueryHeader::new();
        let query = Query::new(true);
        let services = services::CryptoGetAccountRecordsQuery {
            header: None,
            account_id: None,
        };
        AccountRecordsQuery {
            query,
            header,
            services,
        }
    }

    gen_query_account_id_fns!();

    gen_query_execute_with_cost_check!(
        Vec<TransactionRecord>,
        CryptoGetAccountRecords,
        (|res: services::CryptoGetAccountRecordsResponse| {
            res.records
                .into_iter()
                .map(TransactionRecord::try_from)
                .collect::<Result<Vec<TransactionRecord>, crate::error::HederaError>>()
        })
    );
}
