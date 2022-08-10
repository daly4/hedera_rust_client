use std::collections::HashMap;

use crate::error::HederaError;
use crate::ledger_id::LedgerId;
use crate::managed_network::{ArcNetworkNode, ManagedNetwork, NetworkNode};
use crate::network_name::NetworkName;
use crate::node::Node;
use crate::node_address::NodeAddress;
use crate::node_address_book::NodeAddressBook;
use crate::AccountId;

fn mainnet_nodes() -> HashMap<String, AccountId> {
    let mut nodes = HashMap::new();
    nodes.insert("35.237.200.180:50212".to_string(), AccountId::simple(3));
    nodes.insert("35.186.191.247:50212".to_string(), AccountId::simple(4));
    nodes.insert("35.192.2.25:50212".to_string(), AccountId::simple(5));
    nodes.insert("35.199.161.108:50212".to_string(), AccountId::simple(6));
    nodes.insert("35.203.82.240:50212".to_string(), AccountId::simple(7));
    nodes.insert("35.236.5.219:50212".to_string(), AccountId::simple(8));
    nodes.insert("35.197.192.225:50212".to_string(), AccountId::simple(9));
    nodes.insert("35.242.233.154:50212".to_string(), AccountId::simple(10));
    nodes.insert("35.240.118.96:50212".to_string(), AccountId::simple(11));
    nodes.insert("35.204.86.32:50212".to_string(), AccountId::simple(12));
    nodes.insert("35.234.132.107:50212".to_string(), AccountId::simple(13));
    nodes.insert("35.236.2.27:50212".to_string(), AccountId::simple(14));
    nodes.insert("35.228.11.53:50212".to_string(), AccountId::simple(15));
    nodes.insert("34.91.181.183:50212".to_string(), AccountId::simple(16));
    nodes.insert("34.86.212.247:50212".to_string(), AccountId::simple(17));
    nodes.insert("172.105.247.67:50212".to_string(), AccountId::simple(18));
    nodes.insert("34.89.87.138:50212".to_string(), AccountId::simple(19));
    nodes.insert("34.82.78.255:50212".to_string(), AccountId::simple(20));
    nodes
}

fn testnet_nodes() -> HashMap<String, AccountId> {
    let mut nodes = HashMap::new();
    nodes.insert(
        "0.testnet.hedera.com:50211".to_string(),
        AccountId::simple(3),
    );
    nodes.insert(
        "1.testnet.hedera.com:50211".to_string(),
        AccountId::simple(4),
    );
    nodes.insert(
        "2.testnet.hedera.com:50211".to_string(),
        AccountId::simple(5),
    );
    nodes.insert(
        "3.testnet.hedera.com:50211".to_string(),
        AccountId::simple(6),
    );
    nodes.insert(
        "4.testnet.hedera.com:50211".to_string(),
        AccountId::simple(7),
    );
    nodes
}

fn previewnet_nodes() -> HashMap<String, AccountId> {
    let mut nodes = HashMap::new();
    nodes.insert(
        "0.previewnet.hedera.com:50211".to_string(),
        AccountId::simple(3),
    );
    nodes.insert(
        "1.previewnet.hedera.com:50211".to_string(),
        AccountId::simple(4),
    );
    nodes.insert(
        "2.previewnet.hedera.com:50211".to_string(),
        AccountId::simple(5),
    );
    nodes.insert(
        "3.previewnet.hedera.com:50211".to_string(),
        AccountId::simple(6),
    );
    nodes.insert(
        "4.previewnet.hedera.com:50211".to_string(),
        AccountId::simple(7),
    );
    nodes
}

#[derive(Debug, Clone)]
pub struct Network {
    network: ManagedNetwork,
    address_book: Option<HashMap<AccountId, NodeAddress>>,
}

impl Network {
    pub fn new() -> Network {
        Network {
            network: ManagedNetwork::new_empty(),
            address_book: None,
        }
    }

    pub fn ledger_id(&self) -> LedgerId {
        self.network.ledger_id.clone()
    }

    pub async fn set_ledger_id(&mut self, id: LedgerId) -> Result<(), HederaError> {
        self.network.ledger_id = id.clone();
        if self.network.transport_security {
            let address_book = Self::read_address_book_resource(&format!(
                "addressbook/{}.services",
                id.to_string()
            ))
            .await?;
            self.network.set_nodes_address_book(&address_book).await?;
            self.address_book = Some(address_book);
        }
        Ok(())
    }

    pub async fn set_network_name(&mut self, net: NetworkName) -> Result<(), HederaError> {
        let id = LedgerId::from_network_name(net);
        self.set_ledger_id(id).await
    }

    pub async fn read_address_book_resource(
        file_name: &str,
    ) -> Result<HashMap<AccountId, NodeAddress>, HederaError> {
        let f = tokio::fs::read(file_name)
            .await
            .map_err(|_| HederaError::NodeAddressBookDeserialize)?;
        let node_address_book = NodeAddressBook::from_proto_bytes(f)?;
        let mut map: HashMap<AccountId, NodeAddress> = HashMap::new();
        for node_address in node_address_book.node_addresses.into_iter() {
            if let Some(ref account_id) = node_address.account_id {
                map.insert(account_id.clone(), node_address.clone());
            }
        }
        Ok(map)
    }

    pub fn for_network_name(network_name: &NetworkName) -> Result<Self, HederaError> {
        let network = match network_name {
            NetworkName::MainNet => mainnet_nodes(),
            NetworkName::TestNet => testnet_nodes(),
            NetworkName::PreviewNet => previewnet_nodes(),
            NetworkName::Other => HashMap::new(),
        };
        Self::from_network(network)
    }

    pub fn from_network(network: HashMap<String, AccountId>) -> Result<Self, HederaError> {
        let mut cli = Self::new();
        let mut nodes = Vec::with_capacity(network.len());
        for (address, account_id) in network.into_iter() {
            nodes.push(NetworkNode::Node(Node::new(account_id, address, 0)));
        }
        let network = ManagedNetwork::from_nodes(nodes)?;
        cli.network = network;
        Ok(cli)
    }

    pub async fn set_network(
        &self,
        network: HashMap<String, AccountId>,
    ) -> Result<(), HederaError> {
        let mut nodes = Vec::with_capacity(network.len());
        for (address, account_id) in network.into_iter() {
            nodes.push(NetworkNode::Node(Node::new(account_id, address, 0)));
        }
        self.network.set_network_nodes(nodes).await
    }

    pub async fn network(&self) -> Result<HashMap<String, AccountId>, HederaError> {
        self.network.network().await
    }

    pub async fn set_transport_security(&mut self, security: bool) {
        self.network.set_transport_security(security).await;
    }

    pub fn set_max_node_attempts(&mut self, max_node_attempts: u64) {
        self.network.max_node_attempts = max_node_attempts;
    }

    pub fn max_node_attempts(&self) -> u64 {
        self.network.max_node_attempts
    }

    pub fn set_node_min_backoff(&mut self, node_min_backoff: u64) {
        self.network.min_backoff = node_min_backoff;
    }

    pub fn node_min_backoff(&self) -> u64 {
        self.network.min_backoff
    }

    pub fn set_node_max_backoff(&mut self, node_max_backoff: u64) {
        self.network.max_backoff = node_max_backoff;
    }

    pub fn node_max_backoff(&self) -> u64 {
        self.network.max_backoff
    }

    pub fn set_max_nodes_per_transaction(&mut self, max_nodes_per_transaction: usize) {
        self.network.max_nodes_per_transaction = Some(max_nodes_per_transaction);
    }

    pub fn max_nodes_per_transaction(&self) -> Option<usize> {
        self.network.max_nodes_per_transaction
    }

    pub fn set_verify_certificate(&mut self, verify: bool) {
        self.network.verify_certificate = verify;
    }

    pub fn verify_certificate(&self) -> bool {
        self.network.verify_certificate
    }

    pub async fn node_account_ids_for_execute(&self) -> Vec<AccountId> {
        let nodes = self
            .network
            .number_of_most_heathy_nodes(self.network.number_of_nodes_for_transaction().await)
            .await;
        let mut account_ids = Vec::with_capacity(nodes.len());
        for node in nodes {
            let n_r = node.read().await;
            if let NetworkNode::Node(n) = &*n_r {
                account_ids.push(n.account_id.clone());
            }
        }
        account_ids
    }

    pub async fn node_for_account_id(
        &self,
        node_account_id: &AccountId,
    ) -> Result<ArcNetworkNode, HederaError> {
        self.network.node_for_execute(node_account_id).await
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

// }
