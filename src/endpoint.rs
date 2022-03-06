use std::fmt::{self};

use crate::ipv4_address::IPv4Address;
use crate::proto::services::ServiceEndpoint;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Endpoint {
    pub address: IPv4Address,
    pub port: i32,
}

impl From<ServiceEndpoint> for Endpoint {
    fn from(service_endpoint: ServiceEndpoint) -> Endpoint {
        let mut port = service_endpoint.port;
        if port == 0 || port == 50111 {
            port = 50211;
        }
        Endpoint {
            address: IPv4Address::from_proto(service_endpoint.ip_address_v4),
            port: port,
        }
    }
}

impl From<Endpoint> for ServiceEndpoint {
    fn from(endpoint: Endpoint) -> ServiceEndpoint {
        ServiceEndpoint {
            ip_address_v4: endpoint.address.to_proto(),
            port: endpoint.port,
        }
    }
}

impl fmt::Display for Endpoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.address, self.port)
    }
}
