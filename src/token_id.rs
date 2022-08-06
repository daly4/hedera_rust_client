use hedera_derive::{Id, IdPartialEq, IdProto, IdValidateChecksum};
use serde::{Deserialize, Serialize};

use crate::id::IdChecksum;

#[derive(
    Id, IdProto, IdPartialEq, IdValidateChecksum, Debug, Clone, Copy, Hash, Serialize, Deserialize,
)]
#[hedera_derive(field_name = "token_num")]
pub struct TokenId {
    shard_num: i64,
    realm_num: i64,
    token_num: i64,
    checksum: Option<IdChecksum>,
}
