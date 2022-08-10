use num_traits::FromPrimitive;
use std::convert::TryFrom;

use crate::error::HederaError;
use crate::proto::services;
use crate::response_type::ResponseType;
use crate::status::Status;
use crate::Hbar;

#[derive(Debug, Clone, PartialEq)]
pub struct ResponseHeader {
    pub status: Status,
    pub response_type: ResponseType,
    pub cost: Hbar,
    pub state_proof: Vec<u8>,
}

impl TryFrom<services::ResponseHeader> for ResponseHeader {
    type Error = HederaError;

    fn try_from(services: services::ResponseHeader) -> Result<ResponseHeader, HederaError> {
        let status = match Status::from_i32(services.node_transaction_precheck_code) {
            Some(v) => v,
            None => {
                return Err(HederaError::MissingInProto(
                    "node_transaction_precheck_code".to_string(),
                ))
            }
        };
        let response_type = match ResponseType::from_i32(services.response_type) {
            Some(v) => v,
            None => return Err(HederaError::MissingInProto("response_type".to_string())),
        };
        Ok(ResponseHeader {
            status,
            response_type,
            cost: Hbar::try_from(services.cost)?,
            state_proof: services.state_proof,
        })
    }
}
