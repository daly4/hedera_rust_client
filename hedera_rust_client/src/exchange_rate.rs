use std::convert::TryFrom;

use crate::error::HederaError;
use crate::proto::services;

#[derive(Debug, Clone, PartialEq)]
pub struct ExchangeRate {
    pub hbars: i32,
    pub cents: i32,
    pub expiration_time: i64,
}

impl TryFrom<services::ExchangeRate> for ExchangeRate {
    type Error = crate::error::HederaError;
    fn try_from(services: services::ExchangeRate) -> Result<ExchangeRate, HederaError> {
        Ok(ExchangeRate {
            hbars: services.hbar_equiv,
            cents: services.cent_equiv,
            expiration_time: match services.expiration_time {
                Some(v) => v.seconds,
                None => return Err(HederaError::MissingInProto("expiration_time".to_string())),
            },
        })
    }
}
