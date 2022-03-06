use crate::account_id::AccountId;
use crate::endpoint::Endpoint;
use crate::proto::services::{NodeAddress as PbNodeAddress, ServiceEndpoint};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NodeAddress {
    pub public_key: String,
    pub account_id: Option<AccountId>,
    pub node_id: i64,
    pub cert_hash: Vec<u8>,
    pub addresses: Vec<Endpoint>,
    pub description: String,
    pub stake: i64,
}

impl From<PbNodeAddress> for NodeAddress {
    fn from(pb_node_address: PbNodeAddress) -> NodeAddress {
        let mut addresses: Vec<Endpoint> =
            Vec::with_capacity(pb_node_address.service_endpoint.len());
        #[allow(deprecated)]
        if pb_node_address.ip_address.is_empty() {
            let endpoint = ServiceEndpoint {
                ip_address_v4: pb_node_address.ip_address,
                port: pb_node_address.portno,
            };
            addresses.push(endpoint.into());
        }
        for endpoint in pb_node_address.service_endpoint {
            addresses.push(endpoint.into());
        }

        NodeAddress {
            public_key: pb_node_address.rsa_pub_key,
            account_id: pb_node_address
                .node_account_id
                .and_then(|x| x.try_into().ok()),
            node_id: pb_node_address.node_id,
            cert_hash: pb_node_address.node_cert_hash,
            addresses,
            description: pb_node_address.description,
            stake: pb_node_address.stake,
        }
    }
}
