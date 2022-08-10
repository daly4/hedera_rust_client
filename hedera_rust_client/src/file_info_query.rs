use hedera_rust_client_derive::QueryExecuteAsyncWithCostCheck;
use std::convert::TryFrom;

use crate::error::HederaError;
use crate::file_info::FileInfo;
use crate::proto::{services, ToProto};
use crate::query::Query;
use crate::query_header::QueryHeader;

#[derive(QueryExecuteAsyncWithCostCheck, Debug, Clone, PartialEq)]
#[hedera_rust_client_derive(
    proto(proto_enum = "FileGetInfo", response_enum = "FileGetInfo",),
    service(method_service_name = "file", method_service_fn = "get_file_info",)
)]
pub struct FileInfoQuery {
    query: Query,
    header: QueryHeader,
    services: services::FileGetInfoQuery,
}

impl FileInfoQuery {
    pub fn new() -> FileInfoQuery {
        let header = QueryHeader::new();
        let query = Query::new(true);
        let services = services::FileGetInfoQuery {
            header: None,
            file_id: None,
        };
        FileInfoQuery {
            query,
            header,
            services,
        }
    }

    gen_query_file_id_fns!();

    gen_query_execute_with_cost_check!(
        FileInfo,
        FileGetInfo,
        (|res: services::FileGetInfoResponse| {
            if let Some(info) = res.file_info {
                return FileInfo::try_from(info);
            }
            Err(HederaError::MissingInProto("file_id".to_string()))
        })
    );
}
