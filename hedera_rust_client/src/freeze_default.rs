use crate::error::HederaError;
use crate::proto::ToProto;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum FreezeDefault {
    Frozen,
    Unfrozen,
}

impl ToProto<bool> for FreezeDefault {
    fn to_proto(&self) -> std::result::Result<bool, HederaError> {
        Ok(match &self {
            FreezeDefault::Frozen => true,
            FreezeDefault::Unfrozen => false,
        })
    }
}
