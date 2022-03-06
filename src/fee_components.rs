use std::convert::TryFrom;

use crate::error::HederaError;
use crate::proto::services::FeeComponents as ProtoFeeComponents;
use crate::proto::ToProto;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FeeComponents {
    pub min: i64,
    pub max: i64,
    pub constant: i64,
    pub transaction_bandwidth_byte: i64,
    pub transaction_verification: i64,
    pub transaction_ram_byte_hour: i64,
    pub transaction_storage_byte_hour: i64,
    pub contract_transaction_gas: i64,
    pub transfer_volume_hbar: i64,
    pub response_memory_byte: i64,
    pub response_disc_byte: i64,
}

impl TryFrom<ProtoFeeComponents> for FeeComponents {
    type Error = HederaError;
    fn try_from(services: ProtoFeeComponents) -> Result<FeeComponents, Self::Error> {
        Ok(FeeComponents {
            min: services.min,
            max: services.max,
            constant: services.constant,
            transaction_bandwidth_byte: services.bpt,
            transaction_verification: services.vpt,
            transaction_ram_byte_hour: services.rbh,
            transaction_storage_byte_hour: services.sbh,
            contract_transaction_gas: services.gas,
            transfer_volume_hbar: services.tv,
            response_memory_byte: services.bpr,
            response_disc_byte: services.sbpr,
        })
    }
}

impl ToProto<ProtoFeeComponents> for FeeComponents {
    fn to_proto(&self) -> Result<ProtoFeeComponents, HederaError> {
        Ok(ProtoFeeComponents {
            min: self.min,
            max: self.max,
            constant: self.constant,
            bpt: self.transaction_bandwidth_byte,
            vpt: self.transaction_verification,
            rbh: self.transaction_ram_byte_hour,
            sbh: self.transaction_storage_byte_hour,
            gas: self.contract_transaction_gas,
            tv: self.transfer_volume_hbar,
            bpr: self.response_memory_byte,
            sbpr: self.response_disc_byte,
        })
    }
}
