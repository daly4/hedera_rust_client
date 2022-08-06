use crate::error::HederaError;

use crate::proto::{services, ToProto};
use crate::query::Query;
use crate::query_header::QueryHeader;
use crate::LiveHash;
use hedera_derive::QueryExecuteAsyncWithCostCheck;
use std::convert::TryFrom;

#[derive(QueryExecuteAsyncWithCostCheck, Debug, Clone)]
#[hedera_derive(
    proto(proto_enum = "CryptoGetLiveHash", response_enum = "CryptoGetLiveHash",),
    service(method_service_name = "crypto", method_service_fn = "get_live_hash",)
)]
pub struct LiveHashQuery {
    query: Query,
    header: QueryHeader,
    services: services::CryptoGetLiveHashQuery,
}

impl LiveHashQuery {
    pub fn new() -> LiveHashQuery {
        let header = QueryHeader::new();
        let query = Query::new(true);
        let services = services::CryptoGetLiveHashQuery {
            header: None,
            account_id: None,
            hash: Vec::new(),
        };
        LiveHashQuery {
            query,
            header,
            services,
        }
    }

    gen_query_account_id_fns!();

    gen_query_get_set_pb_simple_fns!(hash, Vec<u8>, hash, set_hash);

    gen_query_execute_with_cost_check!(
        LiveHash,
        CryptoGetLiveHash,
        (|res: services::CryptoGetLiveHashResponse| {
            if let Some(hash) = res.live_hash {
                return LiveHash::try_from(hash);
            }
            Err(HederaError::MissingInProto("live_hash".to_string()))
        })
    );
}
