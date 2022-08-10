use byteorder::{BigEndian, ByteOrder};
use bytes::Buf;
use std::convert::TryFrom;

use crate::contract_log_info::ContractLogInfo;
use crate::error::HederaError;
use crate::proto::services;
use crate::utils;
use crate::ContractId;

#[derive(Debug, Clone, PartialEq)]
pub struct ContractFunctionResult {
    pub contract_id: ContractId,
    pub contract_call_result: Vec<u8>,
    pub error_message: String,
    pub bloom: Vec<u8>,
    pub gas_used: u64,
    pub log_info: Vec<ContractLogInfo>,
    pub created_contract_ids: Vec<ContractId>,
}

impl ContractFunctionResult {
    // gets a solidity bool from the result at the given index
    pub fn bool(&self, index: usize) -> bool {
        self.u32(index) == 1
    }

    // gets a solidity address from the result at the given index
    pub fn address(&self, index: usize) -> [u8; 20] {
        let mut res = [0; 20];
        res.copy_from_slice(&self.contract_call_result[(index * 32) + 12..(index * 32) + 32]);
        res
    }

    // gets a solidity int8 from the result at the given index
    pub fn i8(&self, index: usize) -> i8 {
        self.u8(index) as i8
    }

    // gets a solidity int32 from the result at the given index
    pub fn i32(&self, index: usize) -> i32 {
        let buf =
            BigEndian::read_u32(&self.contract_call_result[index * 32 + 28..(index + 1) * 32]);
        buf as i32
    }

    // gets a solidity int64 from the result at the given index
    pub fn i64(&self, index: usize) -> i64 {
        let buf =
            BigEndian::read_u64(&self.contract_call_result[index * 32 + 24..(index + 1) * 32]);
        buf as i64
    }

    // gets a solidity int256 from the result at the given index
    pub fn i256(&self, index: usize) -> [u8; 32] {
        let mut res = [0; 32];
        res.copy_from_slice(&self.contract_call_result[index * 32..index * 32 + 32]);
        res
    }

    // gets a solidity uint8 from the result at the given index
    pub fn u8(&self, index: usize) -> u8 {
        self.contract_call_result[index * 32 + 31]
    }

    // gets a solidity uint32 from the result at the given index
    pub fn u32(&self, index: usize) -> u32 {
        let buf =
            BigEndian::read_u32(&self.contract_call_result[index * 32 + 28..(index + 1) * 32]);
        buf
    }

    // gets a solidity uint64 from the result at the given index
    pub fn u64(&self, index: usize) -> u64 {
        let buf =
            BigEndian::read_u64(&self.contract_call_result[index * 32 + 24..(index + 1) * 32]);
        buf
    }

    // gets a solidity int256 from the result at the given index
    pub fn u256(&self, index: usize) -> [u8; 32] {
        let mut res = [0; 32];
        res.copy_from_slice(&self.contract_call_result[index * 32..index * 32 + 32]);
        res
    }

    pub fn bytes_32(&self, index: usize) -> Vec<u8> {
        self.contract_call_result[index * 32..index * 32 + 32].to_vec()
    }

    // gets a string from the result at the given index
    pub fn string(&self, index: usize) -> String {
        String::from_utf8_lossy(&self.bytes(index)).to_string()
    }

    // gets a byte array from the result at the given index
    pub fn bytes(&self, index: usize) -> Vec<u8> {
        let offset = self.u64(index) as usize;
        let mut buf = &self.contract_call_result[offset + 24..offset + 32];
        let length = buf.get_u64() as usize;
        self.contract_call_result[offset + 32..offset + 32 + length].to_vec()
    }

    // AsBytes returns the raw bytes of the ContractCallResult
    pub fn as_bytes(&self) -> Vec<u8> {
        self.contract_call_result.clone()
    }
}

impl TryFrom<services::ContractFunctionResult> for ContractFunctionResult {
    type Error = HederaError;
    fn try_from(
        services: services::ContractFunctionResult,
    ) -> Result<ContractFunctionResult, Self::Error> {
        let log_info = services
            .log_info
            .into_iter()
            .map(|v| ContractLogInfo::try_from(v))
            .collect::<Result<Vec<ContractLogInfo>, HederaError>>()?;
        #[allow(deprecated)]
        let created_contract_ids = services
            .created_contract_i_ds
            .into_iter()
            .map(|v| ContractId::from(v))
            .collect();
        Ok(ContractFunctionResult {
            contract_id: utils::non_optional_contract_id(services.contract_id)?,
            contract_call_result: services.contract_call_result,
            error_message: services.error_message,
            bloom: services.bloom,
            gas_used: services.gas_used,
            log_info,
            created_contract_ids,
        })
    }
}
