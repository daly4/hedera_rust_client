use std::fmt::{self};

use crate::ipv4_address_part::IPv4AddressPart;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IPv4Address {
    pub network: IPv4AddressPart,
    pub host: IPv4AddressPart,
}

impl IPv4Address {
    pub fn from_proto(bytes: Vec<u8>) -> IPv4Address {
        let network = IPv4AddressPart {
            left: bytes[0],
            right: bytes[1],
        };
        let host = IPv4AddressPart {
            left: bytes[2],
            right: bytes[3],
        };
        IPv4Address { network, host }
    }

    pub fn to_proto(self) -> Vec<u8> {
        vec![
            self.network.left,
            self.network.right,
            self.host.left,
            self.host.right,
        ]
    }
}

impl fmt::Display for IPv4Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}", self.network, self.host)
    }
}
