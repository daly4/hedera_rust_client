use hedera_derive::QueryExecuteAsyncWithCostCheck;
use std::convert::TryFrom;

use crate::account_info::AccountInfo;
use crate::error::HederaError;
use crate::proto::{services, ToProto};
use crate::query::Query;
use crate::query_header::QueryHeader;

#[derive(QueryExecuteAsyncWithCostCheck, Debug, Clone, PartialEq)]
#[hedera_derive(
    proto(proto_enum = "CryptoGetInfo", response_enum = "CryptoGetInfo",),
    service(method_service_name = "crypto", method_service_fn = "get_account_info",)
)]
pub struct AccountInfoQuery {
    query: Query,
    header: QueryHeader,
    services: services::CryptoGetInfoQuery,
}

impl AccountInfoQuery {
    pub fn new() -> AccountInfoQuery {
        let header = QueryHeader::new();
        let query = Query::new(true);
        let services = services::CryptoGetInfoQuery {
            header: None,
            account_id: None,
        };
        AccountInfoQuery {
            query,
            header,
            services,
        }
    }

    gen_query_account_id_fns!();

    gen_query_execute_with_cost_check!(
        AccountInfo,
        CryptoGetInfo,
        (|res: services::CryptoGetInfoResponse| {
            if let Some(info) = res.account_info {
                return AccountInfo::try_from(info);
            }
            Err(HederaError::MissingInProto("account_info".to_string()))
        })
    );
}
