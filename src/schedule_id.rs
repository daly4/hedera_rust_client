use hedera_derive::{Id, IdProto, IdPartialEq, IdValidateChecksum};
use serde::{Deserialize, Serialize};

use crate::id::IdChecksum;

#[derive(Id, IdProto, IdPartialEq, IdValidateChecksum, Debug, Clone, Copy, Hash, Serialize, Deserialize)]
#[hedera_derive(field_name = "schedule_num")]
pub struct ScheduleId {
    shard_num: i64,
    realm_num: i64,
    schedule_num: i64,
    checksum: Option<IdChecksum>,
}
