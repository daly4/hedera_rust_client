use chrono::{DateTime, Utc};
use std::convert::TryFrom;

use crate::error::HederaError;
use crate::proto::mirror;
use crate::topic_message_chunk::TopicMessageChunk;
use crate::utils;
use crate::TransactionId;

#[derive(Debug, Clone)]
pub struct TopicMessage {
    pub consensus_timestamp: DateTime<Utc>,
    pub contents: Vec<u8>,
    pub running_hash: Vec<u8>,
    pub sequence_number: u64,
    pub chunks: Option<Vec<TopicMessageChunk>>,
    pub transaction_id: Option<TransactionId>,
}

impl TopicMessage {
    pub fn of_single(
        response: mirror::ConsensusTopicResponse,
    ) -> Result<TopicMessage, HederaError> {
        Ok(TopicMessage {
            consensus_timestamp: utils::non_optional_timestamp(response.consensus_timestamp)?,
            contents: response.message,
            running_hash: response.running_hash,
            sequence_number: response.sequence_number,
            chunks: None,
            transaction_id: None,
        })
    }

    pub fn of_many(
        responses: Vec<mirror::ConsensusTopicResponse>,
    ) -> Result<TopicMessage, HederaError> {
        let length = responses.len();
        let mut chunks = Vec::with_capacity(length);
        let mut transaction_id: Option<TransactionId> = None;
        let mut messages = Vec::with_capacity(length);
        let mut size = 0usize;
        let mut index;

        let consensus_timestamp =
            utils::non_optional_timestamp(responses[length - 1].consensus_timestamp.clone())?;
        let running_hash = responses[length - 1].running_hash.clone();
        let sequence_number = responses[length - 1].sequence_number;

        for r in responses.into_iter() {
            if let Some(info) = &r.chunk_info {
                if transaction_id.is_none() {
                    if let Some(id) = &info.initial_transaction_id {
                        transaction_id = Some(TransactionId::try_from(id.clone())?);
                    }
                }

                index = usize::try_from(info.number - 1)?;

                chunks[index] = TopicMessageChunk::try_from(r.clone())?;
                messages[index] = r.message.clone();
                size += r.message.len();
            }
        }

        let mut final_message = Vec::with_capacity(size);

        for mut m in messages.into_iter() {
            final_message.append(&mut m)
        }

        Ok(TopicMessage {
            consensus_timestamp,
            running_hash,
            sequence_number,
            contents: final_message,
            chunks: Some(chunks),
            transaction_id,
        })
    }
}
