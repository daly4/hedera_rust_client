use hedera_derive::QueryExecuteAsyncWithCostCheck;
use std::convert::TryFrom;

use crate::contract_function_result::ContractFunctionResult;
use crate::error::HederaError;
use crate::proto::{services, ToProto};
use crate::query::Query;
use crate::query_header::QueryHeader;
use crate::Hbar;

#[derive(QueryExecuteAsyncWithCostCheck, Debug, Clone)]
#[hedera_derive(
    proto(proto_enum = "ContractCallLocal", response_enum = "ContractCallLocal",),
    service(
        method_service_name = "contract",
        method_service_fn = "contract_call_local_method",
    )
)]
pub struct ContractCallQuery {
    query: Query,
    header: QueryHeader,
    services: services::ContractCallLocalQuery,
}

impl ContractCallQuery {
    pub fn new() -> ContractCallQuery {
        let header = QueryHeader::new();
        let query = Query::with_max_query_payment(Hbar::new(2.0));
        #[allow(deprecated)]
        let services = services::ContractCallLocalQuery {
            header: None,
            contract_id: None,
            gas: 0,
            function_parameters: Vec::new(),
            max_result_size: 0,
        };
        ContractCallQuery {
            query,
            header,
            services,
        }
    }

    gen_query_contract_id_fns!();

    gen_query_gas_fns!();

    gen_query_contract_params!(
        function_parameters,
        function_parameters,
        set_function_parameters,
        set_function
    );

    gen_query_execute_with_cost_check!(
        ContractFunctionResult, // returns
        ContractCallLocal,      // services::Response enum
        (|res: services::ContractCallLocalResponse| {
            if let Some(info) = res.function_result {
                return ContractFunctionResult::try_from(info);
            }
            Err(HederaError::MissingInProto("function_result".to_string()))
        })
    );
}
