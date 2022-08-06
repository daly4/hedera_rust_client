use crate::client::Client;
use crate::error::HederaError;
use crate::proto::{mirror, ToProto};
use crate::topic_message::TopicMessage;
use crate::TransactionId;
use num::pow;
use std::cmp::min;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct TopicMessageQuery {
    services: mirror::ConsensusTopicQuery,
}

impl TopicMessageQuery {
    pub fn new() -> TopicMessageQuery {
        TopicMessageQuery {
            services: mirror::ConsensusTopicQuery {
                topic_id: None,
                consensus_start_time: None,
                consensus_end_time: None,
                limit: 0,
            },
        }
    }

    // topic_id
    gen_query_topic_id_fns!();

    // consensus_start_time
    gen_query_consensus_start_time_fns!();

    // consensus_end_time
    gen_query_consensus_end_time_fns!();

    // limit
    gen_query_non_optional!(limit, u64, limit, set_limit);

    pub async fn subscribe<F: Fn(TopicMessage) -> Result<(), HederaError>>(
        &mut self,
        client: &Client,
        on_next: F,
    ) -> Result<(), HederaError> {
        let mut attempt = 0u8;
        let max_attempts = 10u8;
        let node = client.next_mirror_node().await?;
        let mut n_w = node.write().await;
        let sub_client = n_w.mirror_channel();
        drop(n_w);

        let request = sub_client?.subscribe_topic(self.services.clone()).await;

        let mut stream = request
            .map_err(|status| HederaError::ProtoClientFailed(status.code()))?
            .into_inner();

        let mut messages: HashMap<TransactionId, Vec<mirror::ConsensusTopicResponse>> =
            HashMap::new();

        loop {
            let response = match stream.message().await {
                Ok(v) => v,
                Err(status) => {
                    if attempt >= max_attempts {
                        return Err(HederaError::MaxAttempsExceeded(max_attempts));
                    }
                    let code = status.code();
                    if code != tonic::Code::Ok
                        && (code == tonic::Code::Unavailable || code == tonic::Code::NotFound)
                    {
                        let delay = min(250 * pow(u64::from(attempt), 2), 8000);
                        tokio::time::sleep(Duration::from_millis(delay)).await;
                        attempt += 1;
                        continue;
                    }
                    return Err(HederaError::ProtoClientFailed(code));
                }
            };

            if let Some(message) = response {
                if let Some(ref chunk_info) = message.chunk_info {
                    if let Some(ref proto_tx_id) = chunk_info.initial_transaction_id {
                        let tx_id = TransactionId::try_from(proto_tx_id.clone())?;
                        let chunk_total = chunk_info.total as usize;

                        if !messages.contains_key(&tx_id) {
                            messages.insert(tx_id.clone(), Vec::with_capacity(chunk_total));
                        }

                        if let Some(chunks) = messages.get_mut(&tx_id) {
                            chunks.push(message);
                            if chunks.len() == chunk_total {
                                on_next(TopicMessage::of_many(chunks.clone())?)?;
                                messages.remove(&tx_id);
                            }
                        }
                    }
                } else {
                    on_next(TopicMessage::of_single(message)?)?;
                }
            }
        }
    }
}
