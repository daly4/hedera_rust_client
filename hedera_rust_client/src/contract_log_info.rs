use std::convert::TryFrom;

use crate::proto::services;
use crate::utils;
use crate::ContractId;

#[derive(Debug, Clone, PartialEq)]
pub struct ContractLogInfo {
    pub contract_id: ContractId,
    pub bloom: Vec<u8>,
    pub topic: Vec<Vec<u8>>,
    pub data: Vec<u8>,
}

impl TryFrom<services::ContractLoginfo> for ContractLogInfo {
    type Error = crate::error::HederaError;
    fn try_from(services: services::ContractLoginfo) -> Result<ContractLogInfo, Self::Error> {
        Ok(ContractLogInfo {
            contract_id: utils::non_optional_contract_id(services.contract_id)?,
            bloom: services.bloom,
            topic: services.topic,
            data: services.data,
        })
    }
}
