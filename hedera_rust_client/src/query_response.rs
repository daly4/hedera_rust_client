use crate::error::HederaError;
use crate::proto::services;
use crate::response_header::ResponseHeader;

#[derive(Debug, Clone, PartialEq)]
pub struct QueryResponse {
    pub header: ResponseHeader,
    pub services: Option<services::response::Response>,
}

impl QueryResponse {
    pub fn new(
        header: ResponseHeader,
        services: Option<services::response::Response>,
    ) -> QueryResponse {
        QueryResponse { header, services }
    }

    pub fn get_proto(&self) -> Result<&services::response::Response, HederaError> {
        match self.services {
            Some(ref v) => Ok(v),
            None => Err(HederaError::NoResponse),
        }
    }
}
