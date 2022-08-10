use lazy_static::lazy_static;
use regex::Regex;
use std::fmt::{self};
use std::str::FromStr;

use crate::error::HederaError;

lazy_static! {
    static ref HOST_AND_PORT: Regex = Regex::new(r"^(\S+):(\d+)$").unwrap();
}

const PORT_MIRROR_PLAIN: u32 = 5600;
const PORT_MIRROR_TLS: u32 = 443;
const PORT_NODE_PLAIN: u32 = 50211;
const PORT_NODE_TLS: u32 = 50212;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ManagedNodeAddress {
    address: String,
    port: u32,
}

impl ManagedNodeAddress {
    pub fn is_transport_security(&self) -> bool {
        self.port == PORT_NODE_TLS || self.port == PORT_MIRROR_TLS
    }

    pub fn to_insecure(&mut self) {
        let port = if self.port == PORT_NODE_TLS {
            PORT_NODE_PLAIN
        } else if self.port == PORT_MIRROR_TLS {
            PORT_MIRROR_PLAIN
        } else {
            PORT_NODE_PLAIN
        };
        self.port = port;
    }

    pub fn to_secure(&mut self) {
        let port = if self.port == PORT_NODE_PLAIN {
            PORT_NODE_TLS
        } else if self.port == PORT_MIRROR_PLAIN {
            PORT_MIRROR_TLS
        } else {
            PORT_NODE_PLAIN
        };
        self.port = port;
    }
}

impl fmt::Display for ManagedNodeAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.address, self.port)
    }
}

impl FromStr for ManagedNodeAddress {
    type Err = HederaError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let split = HOST_AND_PORT
            .captures(s)
            .ok_or(HederaError::InvalidNodeAddress(s.to_string()))?;
        if split.len() != 3 {
            return Err(HederaError::InvalidNodeAddress(s.to_string()));
        }
        Ok(ManagedNodeAddress {
            address: split.get(1).unwrap().as_str().to_string(),
            port: split.get(2).unwrap().as_str().parse()?,
        })
    }
}
