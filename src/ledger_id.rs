use crate::network_name::NetworkName;
use std::fmt;

#[derive(Debug, Clone)]
pub struct LedgerId {
    pub id: Vec<u8>,
}

impl LedgerId {
    pub fn new(id: Vec<u8>) -> LedgerId {
        LedgerId { id }
    }

    pub fn from_network_name(network_name: NetworkName) -> Self {
        let id = match network_name {
            NetworkName::MainNet => [0],
            NetworkName::TestNet => [1],
            NetworkName::PreviewNet => [2],
            NetworkName::Other => [u8::MAX],
        };
        LedgerId::new(id.to_vec())
    }

    pub fn as_network_name(&self) -> NetworkName {
        let id = self.id[0];
        if id == 0u8 {
            NetworkName::MainNet
        } else if id == 1u8 {
            NetworkName::TestNet
        } else if id == 2u8 {
            NetworkName::PreviewNet
        } else {
            NetworkName::Other
        }
    }

    pub fn for_checksum(&self) -> String {
        let id = self.id[0];
        if id == 0 {
            "0".to_string()
        } else if id == 1 {
            "1".to_string()
        } else if id == 2 {
            "2".to_string()
        } else {
            hex::encode(&self.id)
        }
    }

    pub fn for_mainnet() -> Self {
        LedgerId::new([0].to_vec())
    }

    pub fn for_testnet() -> Self {
        LedgerId::new([1].to_vec())
    }

    pub fn for_previewnet() -> Self {
        LedgerId::new([2].to_vec())
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        self.id.clone()
    }
}

impl fmt::Display for LedgerId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let id = self.id[0];
        let s = if id == 0 {
            "mainnet".to_string()
        } else if id == 1 {
            "testnet".to_string()
        } else if id == 2 {
            "previewnet".to_string()
        } else {
            hex::encode(&self.id)
        };
        write!(f, "{}", s)
    }
}