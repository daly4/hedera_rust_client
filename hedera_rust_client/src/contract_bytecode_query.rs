use hedera_rust_client_derive::QueryExecuteAsyncWithCostCheck;
use std::convert::TryFrom;

use crate::proto::{services, ToProto};
use crate::query::Query;
use crate::query_header::QueryHeader;

#[derive(QueryExecuteAsyncWithCostCheck, Debug, Clone, PartialEq)]
#[hedera_rust_client_derive(
    proto(
        proto_enum = "ContractGetBytecode",
        response_enum = "ContractGetBytecodeResponse",
    ),
    service(
        method_service_name = "contract",
        method_service_fn = "contract_call_local_method",
    )
)]
pub struct ContractByteCodeQuery {
    query: Query,
    header: QueryHeader,
    services: services::ContractGetBytecodeQuery,
}

impl ContractByteCodeQuery {
    pub fn new() -> ContractByteCodeQuery {
        let header = QueryHeader::new();
        let query = Query::new(true);
        let services = services::ContractGetBytecodeQuery {
            header: None,
            contract_id: None,
        };
        ContractByteCodeQuery {
            query,
            header,
            services,
        }
    }

    gen_query_contract_id_fns!();

    gen_query_execute_non_failable_with_cost_check!(
        Vec<u8>,                     // returns
        ContractGetBytecodeResponse, // services::Response enum
        (|res: services::ContractGetBytecodeResponse| res.bytecode)
    );
}
