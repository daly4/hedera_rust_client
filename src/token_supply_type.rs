use num_traits::{FromPrimitive, ToPrimitive};

use crate::error::HederaError;
use crate::proto::ToProto;

#[derive(FromPrimitive, ToPrimitive, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TokenSupplyType {
    Infinite = 0,
    Finite = 1,
}

impl TokenSupplyType {
    pub fn from_proto(services: i32) -> Result<TokenSupplyType, HederaError> {
        TokenSupplyType::from_i32(services).ok_or(HederaError::UnexpectedProtoType)
    }
}

impl ToProto<i32> for TokenSupplyType {
    fn to_proto(&self) -> std::result::Result<i32, HederaError> {
        self.to_i32().ok_or(HederaError::UnexpectedProtoType)
    }
}
