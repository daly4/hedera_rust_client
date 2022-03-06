use crate::error::HederaError;
use crate::managed_network::{ArcNetworkNode, ManagedNetwork, NetworkNode};
use crate::mirror_node::MirrorNode;
use crate::network_name::NetworkName;

pub fn mirror_mainnet_nodes() -> Vec<String> {
    vec!["hcs.mainnet.mirrornode.hedera.com:5600".to_string()]
}

pub fn mirror_testnet_nodes() -> Vec<String> {
    vec!["hcs.testnet.mirrornode.hedera.com:5600".to_string()]
}

pub fn mirror_previewnet_nodes() -> Vec<String> {
    vec!["hcs.previewnet.mirrornode.hedera.com:5600".to_string()]
}

#[derive(Debug, Clone)]
pub struct MirrorNetwork {
    network: ManagedNetwork,
}

impl MirrorNetwork {
    pub fn new() -> MirrorNetwork {
        MirrorNetwork {
            network: ManagedNetwork::new_empty(),
        }
    }

    pub fn for_network_name(network_name: &NetworkName) -> Result<Self, HederaError> {
        let network = match network_name {
            NetworkName::MainNet => mirror_mainnet_nodes(),
            NetworkName::TestNet => mirror_testnet_nodes(),
            NetworkName::PreviewNet => mirror_previewnet_nodes(),
            NetworkName::Other => Vec::new(),
        };
        Self::from_network(network)
    }

    pub fn from_network(network: Vec<String>) -> Result<Self, HederaError> {
        let mut cli = Self::new();
        let mut nodes = Vec::with_capacity(network.len());
        for address in network.into_iter() {
            nodes.push(NetworkNode::Mirror(MirrorNode::new(address, 0)));
        }
        let network = ManagedNetwork::from_nodes(nodes)?;
        cli.network = network;
        Ok(cli)
    }

    pub async fn set_setwork(&mut self, network: Vec<String>) -> Result<(), HederaError> {
        let mut nodes = Vec::with_capacity(network.len());
        for address in network.into_iter() {
            nodes.push(NetworkNode::Mirror(MirrorNode::new(address, 0)));
        }
        self.network.set_network_nodes(nodes).await
    }

    pub async fn set_transport_security(&mut self, security: bool) {
        self.network.set_transport_security(security).await;
    }

    pub async fn get_next_mirror_node(&self) -> Result<ArcNetworkNode, HederaError> {
        let n = self.network.number_of_most_heathy_nodes(1).await;
        if n.is_empty() {
            return Err(HederaError::NoNode);
        }
        Ok(n[0].clone())
    }
}
