use hedera_derive::QueryExecuteAsyncWithCostCheck;
use std::convert::TryFrom;

use crate::contract_info::ContractInfo;
use crate::error::HederaError;
use crate::proto::{services, ToProto};
use crate::query::Query;
use crate::query_header::QueryHeader;
use crate::Hbar;

#[derive(QueryExecuteAsyncWithCostCheck, Debug, Clone, PartialEq)]
#[hedera_derive(
    proto(proto_enum = "ContractGetInfo", response_enum = "ContractGetInfo",),
    service(
        method_service_name = "contract",
        method_service_fn = "get_contract_info",
    )
)]
pub struct ContractInfoQuery {
    query: Query,
    header: QueryHeader,
    services: services::ContractGetInfoQuery,
}

impl ContractInfoQuery {
    pub fn new() -> ContractInfoQuery {
        let header = QueryHeader::new();
        let query = Query::with_max_query_payment(Hbar::new(2.0));
        let services = services::ContractGetInfoQuery {
            header: None,
            contract_id: None,
        };
        ContractInfoQuery {
            query,
            header,
            services,
        }
    }

    gen_query_contract_id_fns!();

    gen_query_execute_with_cost_check!(
        ContractInfo,
        ContractGetInfo,
        (|res: services::ContractGetInfoResponse| {
            if let Some(info) = res.contract_info {
                return ContractInfo::try_from(info);
            }
            Err(HederaError::MissingInProto("contract_info".to_string()))
        })
    );
}
