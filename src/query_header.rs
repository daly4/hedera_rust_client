use num_traits::ToPrimitive;

use crate::error::HederaError;
use crate::proto::{services, ToProto};
use crate::response_type::ResponseType;

#[derive(Debug, Clone)]
pub struct QueryHeader {
    pub payment: Option<services::Transaction>,
    pub response_type: ResponseType,
}

impl QueryHeader {
    pub fn new() -> QueryHeader {
        QueryHeader {
            payment: None,
            response_type: ResponseType::AnswerOnly,
        }
    }
}

impl ToProto<services::QueryHeader> for QueryHeader {
    fn to_proto(&self) -> Result<services::QueryHeader, HederaError> {
        Ok(services::QueryHeader {
            payment: self.payment.clone(),
            response_type: match self.response_type.to_i32() {
                Some(v) => v,
                None => return Err(HederaError::InvalidResponseType),
            },
        })
    }
}
