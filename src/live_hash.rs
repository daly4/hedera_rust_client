use chrono::Duration;
use std::convert::TryFrom;

use crate::error::HederaError;
use crate::key_list::KeyList;
use crate::proto::{services, ToProto};
use crate::utils;
use crate::AccountId;

#[derive(Debug, Clone, PartialEq)]
pub struct LiveHash {
    /// The account to which the livehash is attached
    pub account_id: AccountId,
    /// The SHA-384 hash of a credential or certificate
    pub hash: Vec<u8>,
    /// A list of keys (primitive or threshold), all of which must sign to attach the livehash to an account, and any one of which can later delete it.
    pub keys: Option<KeyList>,
    /// The duration for which the livehash will remain valid
    pub duration: Option<Duration>,
}

impl ToProto<services::LiveHash> for LiveHash {
    fn to_proto(&self) -> Result<services::LiveHash, HederaError> {
        Ok(services::LiveHash {
            account_id: Some(self.account_id.to_proto()?),
            hash: self.hash.clone(),
            keys: match &self.keys {
                Some(v) => Some(v.to_proto()?),
                None => None,
            },
            duration: match &self.duration {
                Some(v) => Some(v.to_proto()?),
                None => None,
            },
        })
    }
}

impl TryFrom<services::LiveHash> for LiveHash {
    type Error = crate::error::HederaError;
    fn try_from(services: services::LiveHash) -> Result<LiveHash, Self::Error> {
        Ok(LiveHash {
            account_id: utils::non_optional_account_id(services.account_id)?,
            hash: services.hash,
            keys: utils::optional_key_list(services.keys)?,
            duration: utils::optional_duration(services.duration)?,
        })
    }
}
