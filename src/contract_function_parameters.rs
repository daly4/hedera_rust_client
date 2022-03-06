use byteorder::{BigEndian, ByteOrder};
use std::convert::TryFrom;

use crate::contract_function_selector::ContractFunctionSelector;
use crate::error::HederaError;

#[derive(Debug, Clone)]
pub struct Argument {
    pub value: Vec<u8>,
    pub dynamic: bool,
}

impl Argument {
    pub fn new() -> Argument {
        Argument {
            value: vec![0; 32],
            dynamic: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ContractFunctionParameters {
    function: ContractFunctionSelector,
    arguments: Vec<Argument>,
}

impl ContractFunctionParameters {
    pub fn new() -> ContractFunctionParameters {
        ContractFunctionParameters {
            function: ContractFunctionSelector::new(None),
            arguments: Vec::new(),
        }
    }

    pub fn add_bool(&mut self, value: bool) -> Result<&mut Self, HederaError> {
        let mut argument = Argument::new();
        if value {
            argument.value[31] = 1
        } else {
            argument.value[31] = 0
        }
        self.function.add_bool();
        self.arguments.push(argument);
        Ok(self)
    }

    pub fn add_function(
        &mut self,
        address: String,
        mut selector: ContractFunctionSelector,
    ) -> Result<&mut Self, HederaError> {
        if address.len() != 40 {
            return Err(HederaError::ContractAddressLength(address));
        }

        let mut argument = Argument::new();

        let address_bytes = hex::decode(&address)?;
        let mut bytes = vec![0, 12];
        bytes.extend_from_slice(&address_bytes[0..20]);

        let function = selector.build(None)?;

        bytes.extend_from_slice(&function[0..4]);
        argument.value = bytes;

        self.function.add_function();
        self.arguments.push(argument);

        Ok(self)
    }

    pub fn add_address(&mut self, value: String) -> Result<&mut Self, HederaError> {
        if value.len() != 40 {
            return Err(HederaError::ContractAddressLength(value));
        }

        let address_bytes = hex::decode(&value)?;
        let mut bytes = vec![0, 12];
        bytes.extend_from_slice(&address_bytes[..]);

        let mut argument = Argument::new();
        argument.value = bytes;

        self.function.add_address();
        self.arguments.push(argument);
        Ok(self)
    }

    pub fn add_i8(&mut self, value: i8) -> Result<&mut Self, HederaError> {
        let mut argument = Argument::new();

        argument.value[31] = u8::try_from(value)?;

        self.function.add_i8();
        self.arguments.push(argument);
        Ok(self)
    }

    pub fn add_i32(&mut self, value: i32) -> Result<&mut Self, HederaError> {
        let mut argument = Argument::new();

        let value = u32::try_from(value)?;
        let mut buf = [0; 4];
        write_u32_vec_len(&mut buf, value, &mut argument.value);

        self.function.add_i32();
        self.arguments.push(argument);
        Ok(self)
    }

    pub fn add_i64(&mut self, value: i64) -> Result<&mut Self, HederaError> {
        let mut argument = Argument::new();

        let value = u64::try_from(value)?;
        let mut buf = [0; 8];
        write_u64_vec_len(&mut buf, value, &mut argument.value);

        self.function.add_i64();
        self.arguments.push(argument);
        Ok(self)
    }

    pub fn add_i256(&mut self, value: Vec<u8>) -> Result<&mut Self, HederaError> {
        let mut argument = Argument::new();

        argument.value = value;

        self.function.add_i256();
        self.arguments.push(argument);
        Ok(self)
    }

    pub fn add_u8(&mut self, value: u8) -> Result<&mut Self, HederaError> {
        let mut argument = Argument::new();

        argument.value[31] = value;

        self.function.add_u8();
        self.arguments.push(argument);
        Ok(self)
    }

    pub fn add_u32(&mut self, value: u32) -> Result<&mut Self, HederaError> {
        let mut argument = Argument::new();

        let mut buf = [0; 4];
        write_u32_vec_len(&mut buf, value, &mut argument.value);

        self.function.add_u32();
        self.arguments.push(argument);
        Ok(self)
    }

    pub fn add_u64(&mut self, value: u64) -> Result<&mut Self, HederaError> {
        let mut argument = Argument::new();

        let mut buf = [0; 8];
        write_u64_vec_len(&mut buf, value, &mut argument.value);

        self.function.add_u64();
        self.arguments.push(argument);
        Ok(self)
    }

    pub fn add_u256(&mut self, value: Vec<u8>) -> Result<&mut Self, HederaError> {
        let mut argument = Argument::new();

        argument.value = value;

        self.function.add_u256();
        self.arguments.push(argument);
        Ok(self)
    }

    pub fn add_address_array(&mut self, value: Vec<String>) -> Result<&mut Self, HederaError> {
        let mut argument = Argument::new();
        argument.dynamic = true;

        let mut result = vec![0; value.len() + 32];

        let mut buf = [0; 8];
        write_u64_vec_len(&mut buf, value.len() as u64, &mut argument.value);

        for (i, v) in value.iter().enumerate() {
            if v.len() != 40 {
                return Err(HederaError::ContractAddressLength(v.to_string()));
            }
            let address_bytes = hex::decode(v)?;
            result[i * 32 + 32 + 12..i * 32 + 32 + 32].copy_from_slice(&address_bytes);
        }

        argument.value = result;

        self.function.add_address_array();
        self.arguments.push(argument);
        Ok(self)
    }

    pub fn add_i32_array(&mut self, value: Vec<i32>) -> Result<&mut Self, HederaError> {
        let mut argument = Argument::new();
        argument.dynamic = true;

        let mut result = vec![0; value.len() + 32];

        let mut buf = [0; 8];
        write_u64_vec_len(&mut buf, value.len() as u64, &mut argument.value);

        let mut buf = [0; 4];
        for (i, v) in value.iter().enumerate() {
            let val = u32::try_from(*v)?;
            BigEndian::write_u32(&mut buf, val);
            result[i * 32 + 32 + 28..i * 32 + 32 + 32].copy_from_slice(&buf);
        }

        argument.value = result;

        self.function.add_i32_array();
        self.arguments.push(argument);
        Ok(self)
    }

    pub fn add_i64_array(&mut self, value: Vec<i64>) -> Result<&mut Self, HederaError> {
        let mut argument = Argument::new();
        argument.dynamic = true;

        let mut result = vec![0; value.len() + 32];

        let mut buf = [0; 8];
        write_u64_vec_len(&mut buf, value.len() as u64, &mut argument.value);

        for (i, v) in value.iter().enumerate() {
            let val = u64::try_from(*v)?;
            BigEndian::write_u64(&mut buf, val);
            result[i * 32 + 32 + 24..i * 32 + 32 + 32].copy_from_slice(&buf);
        }

        argument.value = result;

        self.function.add_i64_array();
        self.arguments.push(argument);
        Ok(self)
    }

    pub fn add_i256_array(&mut self, value: Vec<[u8; 32]>) -> Result<&mut Self, HederaError> {
        let mut argument = Argument::new();
        argument.dynamic = true;

        let mut result = vec![0; value.len() + 32];

        let mut buf = [0; 8];
        write_u64_vec_len(&mut buf, value.len() as u64, &mut argument.value);

        for (i, v) in value.iter().enumerate() {
            result[i * 32 + 32..i * 32 + 32 + 32].copy_from_slice(v);
        }

        argument.value = result;

        self.function.add_i256_array();
        self.arguments.push(argument);
        Ok(self)
    }

    pub fn add_u32_array(&mut self, value: Vec<u32>) -> Result<&mut Self, HederaError> {
        let mut argument = Argument::new();
        argument.dynamic = true;

        let mut result = vec![0; value.len() + 32];

        let mut buf = [0; 8];
        write_u64_vec_len(&mut buf, value.len() as u64, &mut argument.value);

        let mut buf = [0; 4];
        for (i, v) in value.iter().enumerate() {
            BigEndian::write_u32(&mut buf, *v);
            result[i * 32 + 32 + 28..i * 32 + 32 + 32].copy_from_slice(&buf);
        }

        argument.value = result;

        self.function.add_u32_array();
        self.arguments.push(argument);
        Ok(self)
    }

    pub fn add_u64_array(&mut self, value: Vec<u64>) -> Result<&mut Self, HederaError> {
        let mut argument = Argument::new();
        argument.dynamic = true;

        let mut result = vec![0; value.len() + 32];

        let mut buf = [0; 8];
        write_u64_vec_len(&mut buf, value.len() as u64, &mut argument.value);

        for (i, v) in value.iter().enumerate() {
            BigEndian::write_u64(&mut buf, *v);
            result[i * 32 + 32 + 24..i * 32 + 32 + 32].copy_from_slice(&buf);
        }

        argument.value = result;

        self.function.add_u64_array();
        self.arguments.push(argument);
        Ok(self)
    }

    pub fn add_u256_array(&mut self, value: Vec<[u8; 32]>) -> Result<&mut Self, HederaError> {
        let mut argument = Argument::new();
        argument.dynamic = true;

        let mut result = vec![0; value.len() + 32];

        let mut buf = [0; 8];
        write_u64_vec_len(&mut buf, value.len() as u64, &mut argument.value);

        for (i, v) in value.iter().enumerate() {
            result[i * 32 + 32..i * 32 + 32 + 32].copy_from_slice(v);
        }

        argument.value = result;

        self.function.add_u256_array();
        self.arguments.push(argument);
        Ok(self)
    }

    pub fn add_string(&mut self, value: String) -> Result<&mut Self, HederaError> {
        let mut argument = Argument::new();
        argument.dynamic = true;

        let bytes = value.as_bytes();
        let mut buf = [0; 8];
        write_u64_vec_len(&mut buf, bytes.len() as u64, &mut argument.value);
        argument.value.extend_from_slice(&bytes[..]);
        let padding = vec![0; 32 - bytes.len() % 32];
        argument.value.extend_from_slice(&padding[..]);

        self.function.add_string();
        self.arguments.push(argument);
        Ok(self)
    }

    pub fn add_bytes(&mut self, value: Vec<u8>) -> Result<&mut Self, HederaError> {
        let mut argument = Argument::new();
        argument.dynamic = true;

        let mut buf = [0; 8];
        write_u64_vec_len(&mut buf, value.len() as u64, &mut argument.value);
        argument.value.extend_from_slice(&value[..]);
        let padding = vec![0; 32 - value.len() % 32];
        argument.value.extend_from_slice(&padding[..]);

        self.function.add_bytes();
        self.arguments.push(argument);
        Ok(self)
    }

    pub fn add_bytes32(&mut self, value: Vec<u8>) -> Result<&mut Self, HederaError> {
        if value.len() != 32 {
            return Err(HederaError::BytesArrayLength(value.len()));
        }
        let mut argument = Argument::new();
        argument.value = value;

        self.function.add_bytes32();
        self.arguments.push(argument);
        Ok(self)
    }

    pub fn add_bytes_array(&mut self, value: Vec<Vec<u8>>) -> Result<&mut Self, HederaError> {
        let mut argument = Argument::new();
        argument.dynamic = true;
        argument.value = bytes_array(value);

        self.function.add_bytes_array();
        self.arguments.push(argument);
        Ok(self)
    }

    pub fn add_bytes32_array(&mut self, value: Vec<Vec<u8>>) -> Result<&mut Self, HederaError> {
        let mut argument = Argument::new();
        argument.dynamic = true;

        let mut result = vec![0; value.len() + 32];

        let mut buf = [0; 8];
        write_u64_vec_len(&mut buf, value.len() as u64, &mut argument.value);

        for (i, v) in value.iter().enumerate() {
            result[i * 32 + 32..i * 32 + 32 + 32].copy_from_slice(&v[0..32]);
        }

        argument.value = result;

        self.function.add_bytes32_array();
        self.arguments.push(argument);
        Ok(self)
    }

    pub fn add_string_array(&mut self, value: Vec<String>) -> Result<&mut Self, HederaError> {
        let mut argument = Argument::new();
        argument.dynamic = true;

        let mut bytes = Vec::new();
        for s in value.iter() {
            bytes.push(s.as_bytes().to_vec());
        }

        argument.value = bytes_array(bytes);

        self.function.add_string_array();
        self.arguments.push(argument);
        Ok(self)
    }

    pub fn build(&mut self, function_name: Option<String>) -> Result<Vec<u8>, HederaError> {
        let mut length = 0usize;

        let mut function_offset = 0usize;
        if function_name.is_some() {
            function_offset = 4;
        }

        for argument in self.arguments.iter() {
            length += 32;
            if argument.dynamic {
                length += argument.value.len();
            }
        }

        let mut result = vec![0; length + function_offset];
        if function_name.is_some() {
            function_offset = 4;
            result[0..4].copy_from_slice(&self.function.build(function_name)?);
        }

        let mut offset = self.arguments.len() * 32;

        let mut buf = [0; 8];
        for (i, argument) in self.arguments.iter().enumerate() {
            if argument.dynamic {
                BigEndian::write_u64(&mut buf, offset as u64);
                result[(i * 32 + function_offset) + 24..(i + 1) * 32 + function_offset]
                    .copy_from_slice(&buf);
                offset += argument.value.len();
            } else {
                result[i * 32 + function_offset..((i + 1) * 32) + function_offset]
                    .copy_from_slice(&argument.value);
            }
        }
        Ok(result)
    }
}

fn write_u64_vec_len(buf: &mut [u8; 8], len: u64, value: &mut Vec<u8>) {
    BigEndian::write_u64(buf, len);
    value[24..32].copy_from_slice(buf);
}

fn write_u32_vec_len(buf: &mut [u8; 4], len: u32, value: &mut Vec<u8>) {
    BigEndian::write_u32(buf, len);
    value[28..32].copy_from_slice(buf);
}

fn bytes_array(value: Vec<Vec<u8>>) -> Vec<u8> {
    let mut length = 0usize;
    for s in value.iter() {
        length += 32 + 32;
        if s.len() / 32 == 0 {
            length += 32;
        } else {
            length += ((s.len() / 32) + 1) * 32;
        }
    }

    // Zero initialize final resulting byte array
    let mut result = vec![0; length + 32];

    // Write length of array into the first 32 bytes
    let mut buf = [0; 8];
    write_u64_vec_len(&mut buf, value.len() as u64, &mut result);

    // Create array of byte arrays to hold each string value
    // Needed to concat later
    let mut arguments = vec![Vec::new(); value.len()];

    // Convert each argument into bytes, and push each argument
    // into the argument list
    for (i, s) in value.iter().enumerate() {
        // Get the length of the current argument (again)
        if s.len() / 32 == 0 {
            length = 32;
        } else {
            length = ((s.len() / 32) + 1) * 32;
        }

        // Create byte array of correct size
        // Length of value to the nearest 32 byte boundry +
        // 32 bytes to store the length
        let mut bytes = vec![0; length + 32];

        // Write length into first 32 bytes
        write_u64_vec_len(&mut buf, s.len() as u64, &mut bytes);

        // Copy string as bytes to the rest of the buffer
        bytes[32..].copy_from_slice(s);

        // Set the argument bytes to be used later
        arguments[i] = bytes;
    }

    // Initialize offset to the number of strings
    let mut offset = value.len() * 32;

    // For each argument, write the offset into result
    // and the argument value (which includes data and length already)
    for (i, s) in arguments.iter().enumerate() {
        BigEndian::write_u64(&mut buf, offset as u64);
        result[(i + 1) * 32 + 24..(i + 2) * 32].copy_from_slice(&buf);
        result[offset + 32..offset + 32 + s.len()].copy_from_slice(s);
        offset += s.len();
    }
    result
}
