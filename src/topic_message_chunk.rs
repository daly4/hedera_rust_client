use chrono::{DateTime, Utc};
use std::convert::TryFrom;

use crate::error::HederaError;
use crate::proto::mirror;
use crate::utils;

#[derive(Debug, Clone)]
pub struct TopicMessageChunk {
    pub consensus_timestamp: DateTime<Utc>,
    pub content_size: usize,
    pub running_hash: Vec<u8>,
    pub sequence_number: u64,
}

impl TryFrom<mirror::ConsensusTopicResponse> for TopicMessageChunk {
    type Error = HederaError;
    fn try_from(
        services: mirror::ConsensusTopicResponse,
    ) -> Result<TopicMessageChunk, Self::Error> {
        Ok(TopicMessageChunk {
            consensus_timestamp: utils::non_optional_timestamp(services.consensus_timestamp)?,
            content_size: services.message.len(),
            running_hash: services.running_hash,
            sequence_number: services.sequence_number,
        })
    }
}
