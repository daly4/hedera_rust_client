use hedera_derive::QueryExecuteAsyncWithCostCheck;
use std::convert::TryFrom;

use crate::error::HederaError;
use crate::proto::{services, ToProto};
use crate::query::Query;
use crate::query_header::QueryHeader;
use crate::token_nft_info::TokenNftInfo;

#[derive(QueryExecuteAsyncWithCostCheck, Debug, Clone)]
#[hedera_derive(
    proto(proto_enum = "TokenGetNftInfo", response_enum = "TokenGetNftInfo",),
    service(
        method_service_name = "token",
        method_service_fn = "get_token_nft_info",
    )
)]
pub struct TokenNftInfoQuery {
    query: Query,
    header: QueryHeader,
    services: services::TokenGetNftInfoQuery,
}

impl TokenNftInfoQuery {
    pub fn new() -> TokenNftInfoQuery {
        let header = QueryHeader::new();
        let query = Query::new(true);
        let services = services::TokenGetNftInfoQuery {
            header: None,
            nft_id: None,
        };
        TokenNftInfoQuery { query, header, services }
    }

    gen_query_nft_id_fns!();

    gen_query_execute_with_cost_check!(
        TokenNftInfo,
        TokenGetNftInfo,
        (|res: services::TokenGetNftInfoResponse| {
            if let Some(info) = res.nft {
                return TokenNftInfo::try_from(info);
            }
            Err(HederaError::MissingInProto("nft".to_string()))
        })
    );
}
