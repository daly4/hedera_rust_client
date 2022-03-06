use chrono::{DateTime, Utc, Duration};
use std::convert::TryFrom;

use crate::error::HederaError;
use crate::key::Key;
use crate::proto::services;
use crate::utils;
use crate::AccountId;
use crate::TopicId;

#[derive(Debug, Clone)]
pub struct TopicInfo {
    /// ID of the token instance
    pub topic_id: TopicId,
    /// The memo associated with the topic (UTF-8 encoding max 100 bytes)
    pub topic_memo: String,
    /// When a topic is created, its running hash is initialized to 48 bytes of binary zeros.
    /// For each submitted message, the topic's running hash is then updated to the output
    /// of a particular SHA-384 digest whose input data include the previous running hash.
    ///
    /// See the TransactionReceipt.proto documentation for an exact description of the
    /// data included in the SHA-384 digest used for the update.
    pub running_hash: Vec<u8>,
    /// Sequence number (starting at 1 for the first submitMessage) of messages on the topic.
    pub sequence_number: u64,
    /// Effective consensus timestamp at (and after) which submitMessage calls will no longer succeed on the topic
    /// and the topic will expire and after AUTORENEW_GRACE_PERIOD be automatically deleted.
    pub expiration_time: Option<DateTime<Utc>>,
    /// Access control for update/delete of the topic. Null if there is no key.
    pub admin_key: Option<Key>,
    /// Access control for ConsensusService.submitMessage. Null if there is no key.
    pub submit_key: Option<Key>,
    /// If an auto-renew account is specified, when the topic expires, its lifetime will be extended by up to this duration (depending on the solvency of the auto-renew account). If the auto-renew account has no funds at all, the topic will be deleted instead.
    pub auto_renew_period: Option<Duration>,
    /// The account, if any, to charge for automatic renewal of the topic's lifetime upon expiry.
    pub auto_renew_account: Option<AccountId>,
}

impl TryFrom<services::ConsensusGetTopicInfoResponse> for TopicInfo {
    type Error = HederaError;
    fn try_from(services: services::ConsensusGetTopicInfoResponse) -> Result<TopicInfo, Self::Error> {
        if let Some(info) = services.topic_info {
            return Ok(TopicInfo {
                topic_id: utils::non_optional_topic_id(services.topic_id)?,
                topic_memo: info.memo,
                running_hash: info.running_hash,
                sequence_number: info.sequence_number,
                expiration_time: utils::optional_timestamp(info.expiration_time)?,
                admin_key: utils::optional_key(info.admin_key)?,
                submit_key: utils::optional_key(info.submit_key)?,
                auto_renew_period: utils::optional_duration(info.auto_renew_period)?,
                auto_renew_account: utils::optional_account_id(info.auto_renew_account)?,
            });
        }
        Err(HederaError::MissingInProto("topic_info".to_string()))
    }
}
