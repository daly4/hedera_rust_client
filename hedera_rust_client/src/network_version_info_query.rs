use hedera_rust_client_derive::QueryExecuteAsyncWithCostCheck;
use std::convert::TryFrom;

use crate::network_version_info::NetworkVersionInfo;
use crate::proto::{services, ToProto};
use crate::query::Query;
use crate::query_header::QueryHeader;

#[derive(QueryExecuteAsyncWithCostCheck, Debug, Clone, PartialEq)]
#[hedera_rust_client_derive(
    proto(
        proto_enum = "NetworkGetVersionInfo",
        response_enum = "NetworkGetVersionInfo",
    ),
    service(
        method_service_name = "network",
        method_service_fn = "get_version_info",
    )
)]
pub struct NetworkVersionInfoQuery {
    query: Query,
    header: QueryHeader,
    services: services::NetworkGetVersionInfoQuery,
}

impl NetworkVersionInfoQuery {
    pub fn new() -> NetworkVersionInfoQuery {
        let header = QueryHeader::new();
        let query = Query::new(true);
        let services = services::NetworkGetVersionInfoQuery { header: None };
        NetworkVersionInfoQuery {
            query,
            header,
            services,
        }
    }

    gen_query_execute_with_cost_check!(
        NetworkVersionInfo,
        NetworkGetVersionInfo,
        (|res: services::NetworkGetVersionInfoResponse| { NetworkVersionInfo::try_from(res) })
    );
}
