use num_traits::{FromPrimitive, ToPrimitive};

use crate::error::HederaError;
use crate::proto::ToProto;

#[derive(FromPrimitive, ToPrimitive, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TokenType {
    FungibleCommon = 0,
    NonFungibleUnique = 1,
}

impl TokenType {
    pub fn from_proto(services: i32) -> Result<TokenType, HederaError> {
        TokenType::from_i32(services).ok_or(HederaError::UnexpectedProtoType)
    }
}

impl ToProto<i32> for TokenType {
    fn to_proto(&self) -> Result<i32, HederaError> {
        self.to_i32().ok_or(HederaError::UnexpectedProtoType)
    }
}
