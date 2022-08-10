use hedera_rust_client_derive::QueryExecuteAsyncWithCostCheck;
use std::convert::TryFrom;

use crate::proto::{services, ToProto};
use crate::query::Query;
use crate::query_header::QueryHeader;

#[derive(QueryExecuteAsyncWithCostCheck, Debug, Clone, PartialEq)]
#[hedera_rust_client_derive(
    proto(proto_enum = "FileGetContents", response_enum = "FileGetContents",),
    service(method_service_name = "file", method_service_fn = "get_file_content",)
)]
pub struct FileContentsQuery {
    query: Query,
    header: QueryHeader,
    services: services::FileGetContentsQuery,
}

impl FileContentsQuery {
    pub fn new() -> FileContentsQuery {
        let header = QueryHeader::new();
        let query = Query::new(true);
        let services = services::FileGetContentsQuery {
            header: None,
            file_id: None,
        };
        FileContentsQuery {
            query,
            header,
            services,
        }
    }

    gen_query_file_id_fns!();

    gen_query_execute_non_failable_with_cost_check!(
        Option<Vec<u8>>,
        FileGetContents,
        (|res: services::FileGetContentsResponse| {
            match res.file_contents {
                Some(v) => Some(v.contents),
                None => None,
            }
        })
    );
}
