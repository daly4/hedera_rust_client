use hedera_derive::{Id, IdPartialEq, IdProto, IdValidateChecksum};
use serde::{Deserialize, Serialize};

use crate::id::IdChecksum;

#[derive(
    Id, IdProto, IdPartialEq, IdValidateChecksum, Debug, Clone, Copy, Hash, Serialize, Deserialize,
)]
#[hedera_derive(field_name = "file_num")]
pub struct FileId {
    shard_num: i64,
    realm_num: i64,
    file_num: i64,
    checksum: Option<IdChecksum>,
}
