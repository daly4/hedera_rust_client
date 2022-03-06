use hedera_derive::QueryExecuteAsyncWithCostCheck;
use std::convert::TryFrom;

use crate::proto::{services, ToProto};
use crate::query::Query;
use crate::query_header::QueryHeader;
use crate::transfer::Transfer;

#[derive(QueryExecuteAsyncWithCostCheck, Debug, Clone)]
#[hedera_derive(
    proto(
        proto_enum = "CryptoGetProxyStakers",
        response_enum = "CryptoGetProxyStakers",
    ),
    service(
        method_service_name = "crypto",
        method_service_fn = "get_stakers_by_account_id",
    )
)]
pub struct AccountStakersQuery {
    query: Query,
    header: QueryHeader,
    services: services::CryptoGetStakersQuery,
}

impl AccountStakersQuery {
    pub fn new() -> AccountStakersQuery {
        let header = QueryHeader::new();
        let query = Query::new(true);
        let services = services::CryptoGetStakersQuery {
            header: None,
            account_id: None,
        };
        AccountStakersQuery { query, header, services }
    }

    gen_query_account_id_fns!();

    gen_query_execute_with_cost_check!(
        Option<Vec<Transfer>>,
        CryptoGetProxyStakers,
        (|res: services::CryptoGetStakersResponse| {
            match res.stakers {
                Some(val) => {
                    let stakers = val
                        .proxy_staker
                        .into_iter()
                        .map(|v| Transfer::try_from(v))
                        .collect::<Result<Vec<Transfer>, crate::error::HederaError>>();
                    match stakers {
                        Ok(v) => Ok(Some(v)),
                        Err(e) => Err(e),
                    }
                }
                None => Ok(None),
            }
        })
    );
}
