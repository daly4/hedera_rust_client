use hedera_derive::QueryExecuteAsyncWithCostCheck;
use std::convert::TryFrom;

use crate::error::HederaError;
use crate::proto::{services, ToProto};
use crate::query::Query;
use crate::query_header::QueryHeader;
use crate::schedule_info::ScheduleInfo;

#[derive(QueryExecuteAsyncWithCostCheck, Debug, Clone)]
#[hedera_derive(
    proto(proto_enum = "ScheduleGetInfo", response_enum = "ScheduleGetInfo",),
    service(
        method_service_name = "schedule",
        method_service_fn = "get_schedule_info",
    )
)]
pub struct ScheduleInfoQuery {
    query: Query,
    header: QueryHeader,
    services: services::ScheduleGetInfoQuery,
}

impl ScheduleInfoQuery {
    pub fn new() -> ScheduleInfoQuery {
        let header = QueryHeader::new();
        let query = Query::new(true);
        let services = services::ScheduleGetInfoQuery {
            header: None,
            schedule_id: None,
        };
        ScheduleInfoQuery { query, header, services }
    }

    gen_query_schedule_id_fns!();

    gen_query_execute_with_cost_check!(
        ScheduleInfo,
        ScheduleGetInfo,
        (|res: services::ScheduleGetInfoResponse| {
            if let Some(info) = res.schedule_info {
                return ScheduleInfo::try_from(info);
            }
            Err(HederaError::MissingInProto("schedule_info".to_string()))
        })
    );
}
