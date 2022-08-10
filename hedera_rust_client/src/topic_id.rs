use hedera_rust_client_derive::{Id, IdPartialEq, IdProto, IdValidateChecksum};
use serde::{Deserialize, Serialize};

use crate::id::IdChecksum;

#[derive(
    Id, IdProto, IdPartialEq, IdValidateChecksum, Debug, Clone, Copy, Hash, Serialize, Deserialize,
)]
#[hedera_rust_client_derive(field_name = "topic_num")]
pub struct TopicId {
    shard_num: i64,
    realm_num: i64,
    topic_num: i64,
    checksum: Option<IdChecksum>,
}
