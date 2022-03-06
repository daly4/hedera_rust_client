use crate::proto::{services, ToProto};
use crate::query::Query;
use crate::query_header::QueryHeader;
use crate::topic_info::TopicInfo;
use hedera_derive::QueryExecuteAsyncWithCostCheck;
use std::convert::TryFrom;

#[derive(QueryExecuteAsyncWithCostCheck, Debug, Clone)]
#[hedera_derive(
    proto(
        proto_enum = "ConsensusGetTopicInfo",
        response_enum = "ConsensusGetTopicInfo",
    ),
    service(method_service_name = "topic", method_service_fn = "get_topic_info",)
)]
pub struct TopicInfoQuery {
    query: Query,
    header: QueryHeader,
    services: services::ConsensusGetTopicInfoQuery,
}

impl TopicInfoQuery {
    pub fn new() -> TopicInfoQuery {
        let header = QueryHeader::new();
        let query = Query::new(true);
        let services = services::ConsensusGetTopicInfoQuery {
            header: None,
            topic_id: None,
        };
        TopicInfoQuery { query, header, services }
    }

    gen_query_topic_id_fns!();

    gen_query_execute_with_cost_check!(
        TopicInfo,
        ConsensusGetTopicInfo,
        (|res: services::ConsensusGetTopicInfoResponse| { TopicInfo::try_from(res) })
    );
}
