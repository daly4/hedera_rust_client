use hedera_derive::QueryExecuteAsyncWithCostCheck;
use std::convert::TryFrom;

use crate::error::HederaError;
use crate::proto::{services, ToProto};
use crate::query::Query;
use crate::query_header::QueryHeader;
use crate::token_info::TokenInfo;

#[derive(QueryExecuteAsyncWithCostCheck, Debug, Clone, PartialEq)]
#[hedera_derive(
    proto(proto_enum = "TokenGetInfo", response_enum = "TokenGetInfo",),
    service(method_service_name = "token", method_service_fn = "get_token_info",)
)]
pub struct TokenInfoQuery {
    query: Query,
    header: QueryHeader,
    services: services::TokenGetInfoQuery,
}

impl TokenInfoQuery {
    pub fn new() -> TokenInfoQuery {
        let header = QueryHeader::new();
        let query = Query::new(true);
        let services = services::TokenGetInfoQuery {
            header: None,
            token: None,
        };
        TokenInfoQuery {
            query,
            header,
            services,
        }
    }

    gen_query_token_id_fns!();

    gen_query_execute_with_cost_check!(
        TokenInfo,
        TokenGetInfo,
        (|res: services::TokenGetInfoResponse| {
            if let Some(info) = res.token_info {
                return TokenInfo::try_from(info);
            }
            Err(HederaError::MissingInProto("token_info".to_string()))
        })
    );
}
