use crate::error::HederaError;
use sha3::{Digest, Keccak256};

#[derive(Debug, Clone, PartialEq)]
pub enum ArgumentType {
    Bool,
    String,
    I8,
    I32,
    I64,
    I256,
    U8,
    U32,
    U64,
    U256,
    Bytes,
    Bytes32,
    Function,
    Address,
}

impl ArgumentType {
    fn value(&self) -> &str {
        match *self {
            ArgumentType::Bool => "bool",
            ArgumentType::String => "string",
            ArgumentType::I8 => "int8",
            ArgumentType::I32 => "int32",
            ArgumentType::I64 => "int64",
            ArgumentType::I256 => "int256",
            ArgumentType::U8 => "uint8",
            ArgumentType::U32 => "uint32",
            ArgumentType::U64 => "uint64",
            ArgumentType::U256 => "uint256",
            ArgumentType::Bytes => "bytes",
            ArgumentType::Bytes32 => "bytes32",
            ArgumentType::Function => "function",
            ArgumentType::Address => "address",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Solidity {
    pub ty: ArgumentType,
    pub array: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ContractFunctionSelector {
    pub function: Option<String>,
    pub params: String,
    pub param_types: Vec<Solidity>,
}

impl ContractFunctionSelector {
    pub fn new(name: Option<String>) -> ContractFunctionSelector {
        ContractFunctionSelector {
            function: name,
            params: String::new(),
            param_types: Vec::new(),
        }
    }

    fn add_param(&mut self, ty: Solidity) -> &mut Self {
        if !self.param_types.is_empty() {
            self.params.push(',');
        }
        self.params.push_str(ty.ty.value());
        if ty.array {
            self.params.push_str("[]");
        }
        self.param_types.push(ty);
        self
    }

    pub fn add_function(&mut self) -> &mut Self {
        self.add_param(Solidity {
            ty: ArgumentType::Function,
            array: false,
        });
        self
    }

    pub fn add_address(&mut self) -> &mut Self {
        self.add_param(Solidity {
            ty: ArgumentType::Address,
            array: false,
        });
        self
    }

    pub fn add_bool(&mut self) -> &mut Self {
        self.add_param(Solidity {
            ty: ArgumentType::Bool,
            array: false,
        });
        self
    }

    pub fn add_string(&mut self) -> &mut Self {
        self.add_param(Solidity {
            ty: ArgumentType::String,
            array: false,
        });
        self
    }

    pub fn add_i8(&mut self) -> &mut Self {
        self.add_param(Solidity {
            ty: ArgumentType::I8,
            array: false,
        });
        self
    }

    pub fn add_i32(&mut self) -> &mut Self {
        self.add_param(Solidity {
            ty: ArgumentType::I32,
            array: false,
        });
        self
    }

    pub fn add_i64(&mut self) -> &mut Self {
        self.add_param(Solidity {
            ty: ArgumentType::I64,
            array: false,
        });
        self
    }

    pub fn add_i256(&mut self) -> &mut Self {
        self.add_param(Solidity {
            ty: ArgumentType::I256,
            array: false,
        });
        self
    }

    pub fn add_u8(&mut self) -> &mut Self {
        self.add_param(Solidity {
            ty: ArgumentType::U8,
            array: false,
        });
        self
    }

    pub fn add_u32(&mut self) -> &mut Self {
        self.add_param(Solidity {
            ty: ArgumentType::U32,
            array: false,
        });
        self
    }

    pub fn add_u64(&mut self) -> &mut Self {
        self.add_param(Solidity {
            ty: ArgumentType::U64,
            array: false,
        });
        self
    }

    pub fn add_u256(&mut self) -> &mut Self {
        self.add_param(Solidity {
            ty: ArgumentType::U256,
            array: false,
        });
        self
    }

    pub fn add_bytes(&mut self) -> &mut Self {
        self.add_param(Solidity {
            ty: ArgumentType::Bytes,
            array: false,
        });
        self
    }

    pub fn add_bytes32(&mut self) -> &mut Self {
        self.add_param(Solidity {
            ty: ArgumentType::Bytes32,
            array: false,
        });
        self
    }

    pub fn add_address_array(&mut self) -> &mut Self {
        self.add_param(Solidity {
            ty: ArgumentType::Address,
            array: true,
        });
        self
    }

    pub fn add_bool_array(&mut self) -> &mut Self {
        self.add_param(Solidity {
            ty: ArgumentType::Bool,
            array: true,
        });
        self
    }

    pub fn add_string_array(&mut self) -> &mut Self {
        self.add_param(Solidity {
            ty: ArgumentType::String,
            array: true,
        });
        self
    }

    pub fn add_i8_array(&mut self) -> &mut Self {
        self.add_param(Solidity {
            ty: ArgumentType::I8,
            array: true,
        });
        self
    }

    pub fn add_i32_array(&mut self) -> &mut Self {
        self.add_param(Solidity {
            ty: ArgumentType::I32,
            array: true,
        });
        self
    }

    pub fn add_i64_array(&mut self) -> &mut Self {
        self.add_param(Solidity {
            ty: ArgumentType::I64,
            array: true,
        });
        self
    }

    pub fn add_i256_array(&mut self) -> &mut Self {
        self.add_param(Solidity {
            ty: ArgumentType::I256,
            array: true,
        });
        self
    }

    pub fn add_u8_array(&mut self) -> &mut Self {
        self.add_param(Solidity {
            ty: ArgumentType::U8,
            array: true,
        });
        self
    }

    pub fn add_u32_array(&mut self) -> &mut Self {
        self.add_param(Solidity {
            ty: ArgumentType::U32,
            array: true,
        });
        self
    }

    pub fn add_u64_array(&mut self) -> &mut Self {
        self.add_param(Solidity {
            ty: ArgumentType::U64,
            array: true,
        });
        self
    }

    pub fn add_u256_array(&mut self) -> &mut Self {
        self.add_param(Solidity {
            ty: ArgumentType::U256,
            array: true,
        });
        self
    }

    pub fn add_bytes_array(&mut self) -> &mut Self {
        self.add_param(Solidity {
            ty: ArgumentType::Bytes,
            array: true,
        });
        self
    }

    pub fn add_bytes32_array(&mut self) -> &mut Self {
        self.add_param(Solidity {
            ty: ArgumentType::Bytes32,
            array: true,
        });
        self
    }

    pub fn string(&self) -> String {
        let mut function = String::new();
        if let Some(f) = &self.function {
            function = f.clone();
        }
        function.push('(');
        function.push_str(&self.params);
        function.push(')');
        function
    }

    // Complete the function selector after all parameters have been added and get the selector
    // bytes.
    // No more parameters may be added after this method call.
    // However, this can be called multiple times; it will always return the same result.
    // return the computed selector bytes
    pub fn build(&mut self, function: Option<String>) -> Result<[u8; 5], HederaError> {
        if let Some(f) = function {
            self.function = Some(f);
        } else if self.function.is_none() {
            return Err(HederaError::Unreacahble);
        }

        let mut hasher = Keccak256::new();
        hasher.update(self.string());
        let hash = hasher.finalize();
        let mut res = [0; 5];
        res.copy_from_slice(&hash[0..4]);
        Ok(res)
    }
}
