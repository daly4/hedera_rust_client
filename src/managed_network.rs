#[allow(dead_code)]

use itertools::enumerate;
use std::cmp::min;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;

use crate::error::HederaError;
use crate::AccountId;

use crate::channel::Channel;
use crate::ledger_id::LedgerId;
use crate::managed_node::ManagedNode;
use crate::managed_node_address::ManagedNodeAddress;
use crate::node_address::NodeAddress;
use crate::mirror_node::{ConsensusServiceClientChannel, MirrorNode};
use crate::node::Node;

const MIN_BACKOFF: u64 = 250;
const MAX_BACKOFF: u64 = 8000;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NetworkNodeKey {
    AccountId(AccountId),
    Address(ManagedNodeAddress),
}

#[derive(Debug, Clone)]
pub enum NetworkNode {
    Node(Node),
    Mirror(MirrorNode),
}

impl NetworkNode {
    pub fn account_id(&self) -> Result<AccountId, HederaError> {
        match &self {
            NetworkNode::Node(n) => Ok(n.account_id),
            NetworkNode::Mirror(_) => Err(HederaError::InvalidNodeType),
        }
    }
    pub fn attempts(&self) -> u64 {
        match &self {
            NetworkNode::Node(n) => n.managed_node.attempts,
            NetworkNode::Mirror(n) => n.managed_node.attempts,
        }
    }
    pub fn set_min_backoff(&mut self, wait_time: u64) {
        match self {
            NetworkNode::Node(n) => n.managed_node.min_backoff = wait_time,
            NetworkNode::Mirror(n) => n.managed_node.min_backoff = wait_time,
        }
    }
    pub fn min_backoff(&self) -> u64 {
        match &self {
            NetworkNode::Node(n) => n.managed_node.min_backoff,
            NetworkNode::Mirror(n) => n.managed_node.min_backoff,
        }
    }
    pub fn set_max_backoff(&mut self, wait_time: u64) {
        match self {
            NetworkNode::Node(n) => n.managed_node.max_backoff = wait_time,
            NetworkNode::Mirror(n) => n.managed_node.max_backoff = wait_time,
        }
    }
    pub fn max_backoff(&self) -> u64 {
        match &self {
            NetworkNode::Node(n) => n.managed_node.max_backoff,
            NetworkNode::Mirror(n) => n.managed_node.max_backoff,
        }
    }
    pub fn in_use(&mut self) {
        match self {
            NetworkNode::Node(n) => n.in_use(),
            NetworkNode::Mirror(n) => n.in_use(),
        }
    }
    pub fn is_healthy(&self) -> bool {
        match &self {
            NetworkNode::Node(n) => n.is_healthy(),
            NetworkNode::Mirror(n) => n.is_healthy(),
        }
    }
    pub fn health(&self, idx: usize) -> ManagedNodeHealth {
        match &self {
            NetworkNode::Node(n) => ManagedNodeHealth {
                idx,
                key: NetworkNodeKey::AccountId(n.account_id),
                address: n.managed_node.address.clone(),
                is_healthy: n.is_healthy(),
                last_used: n.managed_node.last_used,
                backoff_until: n.managed_node.backoff_until,
                use_count: n.managed_node.use_count,
                attempts: n.managed_node.attempts,
            },
            NetworkNode::Mirror(n) => ManagedNodeHealth {
                idx,
                key: NetworkNodeKey::Address(n.managed_node.address.clone()),
                address: n.managed_node.address.clone(),
                is_healthy: n.is_healthy(),
                last_used: n.managed_node.last_used,
                backoff_until: n.managed_node.backoff_until,
                use_count: n.managed_node.use_count,
                attempts: n.managed_node.attempts,
            },
        }
    }
    pub fn remaining_time_for_backoff(&self) -> i64 {
        match &self {
            NetworkNode::Node(n) => n.remaining_backoff(), // node only
            NetworkNode::Mirror(_) => 0,
        }
    }
    pub fn increase_delay(&mut self) {
        match self {
            NetworkNode::Node(n) => n.increase_delay(),
            NetworkNode::Mirror(n) => n.increase_delay(),
        }
    }
    pub fn decrease_delay(&mut self) {
        match self {
            NetworkNode::Node(n) => n.decrease_delay(),
            NetworkNode::Mirror(n) => n.decrease_delay(),
        }
    }
    pub fn wait(&self) -> Duration {
        match &self {
            NetworkNode::Node(n) => n.wait(),
            NetworkNode::Mirror(n) => n.wait(),
        }
    }
    pub fn to_secure(&mut self) {
        match self {
            NetworkNode::Node(n) => n.to_secure(),
            NetworkNode::Mirror(n) => n.to_secure(),
        }
    }
    pub fn to_insecure(&mut self) {
        match self {
            NetworkNode::Node(n) => n.to_insecure(),
            NetworkNode::Mirror(n) => n.to_insecure(),
        }
    }
    pub fn managed_node(&self) -> &ManagedNode {
        match &self {
            NetworkNode::Node(n) => &n.managed_node,
            NetworkNode::Mirror(n) => &n.managed_node,
        }
    }
    pub fn address(&self) -> ManagedNodeAddress {
        match &self {
            NetworkNode::Node(n) => n.managed_node.address.clone(),
            NetworkNode::Mirror(n) => n.managed_node.address.clone(),
        }
    }
    pub fn set_node_address_book(&mut self, address_book: NodeAddress) -> Result<(), HederaError> {
        match self {
            NetworkNode::Node(n) => {
                n.address_book = Some(address_book);
                Ok(())
            },
            NetworkNode::Mirror(_) => Err(HederaError::InvalidNodeType),
        }
    }
    pub fn close(&mut self) {
        match self {
            NetworkNode::Node(n) => n.close(),
            NetworkNode::Mirror(n) => n.close(),
        }
    }
    pub fn key(&self) -> NetworkNodeKey {
        match &self {
            NetworkNode::Node(n) => NetworkNodeKey::AccountId(n.account_id),
            NetworkNode::Mirror(n) => NetworkNodeKey::Address(n.managed_node.address.clone()),
        }
    }
    pub fn node_channel(&mut self) -> Result<Channel, HederaError> {
        match self {
            NetworkNode::Node(n) => n.channel(),
            NetworkNode::Mirror(_) => Err(HederaError::InvalidNodeType),
        }
    }
    pub fn mirror_channel(&mut self) -> Result<ConsensusServiceClientChannel, HederaError> {
        match self {
            NetworkNode::Node(_) => Err(HederaError::InvalidNodeType),
            NetworkNode::Mirror(n) => n.channel(),
        }
    }
}

pub type ArcNetworkNode = Arc<RwLock<NetworkNode>>;
pub type ArcNodeVec = Vec<ArcNetworkNode>;
pub type NetworkHashMap = HashMap<NetworkNodeKey, ArcNodeVec>;

#[derive(Debug, Clone)]
pub struct ManagedNetwork {
    pub network: Arc<RwLock<NetworkHashMap>>,
    pub nodes: Arc<RwLock<ArcNodeVec>>,
    pub max_node_attempts: u64,
    pub min_backoff: u64,
    pub max_backoff: u64,
    pub max_nodes_per_transaction: Option<usize>,
    pub ledger_id: LedgerId,
    pub transport_security: bool,
    pub verify_certificate: bool,
}

impl ManagedNetwork {
    pub fn new(
        network: Arc<RwLock<NetworkHashMap>>,
        nodes: Arc<RwLock<ArcNodeVec>>,
    ) -> ManagedNetwork {
        ManagedNetwork {
            network,
            nodes,
            ledger_id: LedgerId::for_mainnet(),
            max_node_attempts: 0,
            min_backoff: MIN_BACKOFF,
            max_backoff: MAX_BACKOFF,
            max_nodes_per_transaction: None,
            transport_security: false,
            verify_certificate: false,
        }
    }

    pub fn new_empty() -> ManagedNetwork {
        Self::new(
            Arc::new(RwLock::new(HashMap::new())),
            Arc::new(RwLock::new(Vec::new())),
        )
    }

    async fn address_is_in_node_list(
        address: &ManagedNodeAddress,
        nodes: &ArcNodeVec,
    ) -> Option<usize> {
        for (i, node) in enumerate(nodes) {
            let n_r = node.read().await;
            let addr = n_r.address();
            drop(n_r);
            if &addr == address {
                return Some(i);
            }
        }
        None
    }

    pub fn from_nodes(from_nodes: Vec<NetworkNode>) -> Result<Self, HederaError> {
        let mut network_nodes: NetworkHashMap = HashMap::new();
        let mut nodes = Vec::with_capacity(from_nodes.len());
        for mut f_node in from_nodes.into_iter() {
            f_node.set_min_backoff(MIN_BACKOFF);
            f_node.set_max_backoff(MAX_BACKOFF);
            let key = f_node.key();
            let node = Arc::new(RwLock::new(f_node));
            nodes.push(Arc::clone(&node));
            match network_nodes.get_mut(&key) {
                Some(n_nodes) => {
                    n_nodes.push(Arc::clone(&node));
                }
                None => {
                    let mut n = Vec::new();
                    n.push(Arc::clone(&node));
                    network_nodes.insert(key, n);
                }
            }
        }
        Ok(Self::new(
            Arc::new(RwLock::new(network_nodes)),
            Arc::new(RwLock::new(nodes)),
        ))
    }

    // make smart-overwrite?
    pub async fn set_network_nodes(&self, from_nodes: Vec<NetworkNode>) -> Result<(), HederaError> {
        let mut network_nodes: NetworkHashMap = HashMap::new();
        let mut nodes = Vec::with_capacity(from_nodes.len());

        for mut f_node in from_nodes.into_iter() {
            f_node.set_min_backoff(self.min_backoff);
            f_node.set_max_backoff(self.max_backoff);
            let address = f_node.address();
            let key = f_node.key();
            let node = Arc::new(RwLock::new(f_node));
            if Self::address_is_in_node_list(&address, &nodes)
                .await
                .is_none()
            {
                nodes.push(Arc::clone(&node));
            }
            match network_nodes.get_mut(&key) {
                Some(n_nodes) => {
                    if !n_nodes.is_empty() {
                        if Self::address_is_in_node_list(&address, n_nodes)
                            .await
                            .is_none()
                        {
                            nodes.push(Arc::clone(&node));
                        }
                    } else {
                        nodes.push(Arc::clone(&node));
                    }
                }
                None => {
                    let mut n = Vec::new();
                    n.push(Arc::clone(&node));
                    network_nodes.insert(key, n);
                }
            }
        }
        let mut current_network = self.network.write().await;
        *current_network = network_nodes;
        drop(current_network);

        let mut current_nodes = self.nodes.write().await;
        *current_nodes = nodes;
        drop(current_nodes);
        Ok(())
    }

    #[allow(dead_code)]
    pub async fn close(&self) {
        let nodes = self.nodes.read().await;
        for node in nodes.iter() {
            let mut n_w = node.write().await;
            n_w.close();
        }
    }

    pub async fn set_transport_security(&mut self, security: bool) {
        if self.transport_security != security {
            let nodes = self.nodes.read().await;
            for node in nodes.iter() {
                let mut n_w = node.write().await;
                n_w.close();
                if security {
                    n_w.to_secure();
                } else {
                    n_w.to_insecure();
                }
            }
            self.transport_security = security;
        }
    }

    pub async fn set_nodes_address_book(&mut self, address_book: &HashMap<AccountId, NodeAddress>) -> Result<(), HederaError> {
        let nodes = self.nodes.read().await;
        for node in nodes.iter() {
            let mut n_w = node.write().await;
            let account_id = n_w.account_id()?;
            let address = address_book.get(&account_id).ok_or(HederaError::InvalidNodeAccountId)?;
            n_w.set_node_address_book(address.clone())?;
        }
        Ok(())
    }    

    pub async fn network(&self) -> Result<HashMap<String, AccountId>, HederaError> {
        let nodes = self.nodes.read().await;
        let mut network = HashMap::with_capacity(nodes.len());
        for node in nodes.iter() {
            let n_r = node.read().await;
            let address = n_r.address().to_string();
            let account_id = n_r.account_id()?;
            drop(n_r);
            network.insert(address, account_id);
        }
        Ok(network)
    }

    pub async fn number_of_nodes_for_transaction(&self) -> usize {
        let network = self.network.read().await;
        let network_len = network.len();
        drop(network);
        if let Some(max_nodes_per_transaction) = self.max_nodes_per_transaction {
            return min(max_nodes_per_transaction, network_len);
        }
        (network_len + 3 - 1) / 3
    }

    pub async fn number_of_most_heathy_nodes(&self, count: usize) -> ArcNodeVec {
        let nodes = self.nodes.read().await;
        let mut node_health = Vec::with_capacity(nodes.len());
        for (i, node) in enumerate(&*nodes) {
            let health = node.read().await.health(i);
            node_health.push(health);
        }
        drop(nodes);

        // order node health vec
        node_health.sort_unstable();
        self.remove_dead_nodes(&node_health).await;

        let nodes = self.nodes.read().await;
        let size = min(count, nodes.len());
        let mut return_nodes = Vec::with_capacity(size);
        for n in 0..size {
            return_nodes.push(nodes[n].clone());
        }
        return_nodes
    }

    pub async fn node_for_execute(
        &self,
        node_account_id: &AccountId,
    ) -> Result<ArcNetworkNode, HederaError> {
        let network = self.network.read().await;
        let node_vec = network
            .get(&NetworkNodeKey::AccountId(node_account_id.clone()))
            .ok_or(HederaError::InvalidNodeAccountId)?;
        let mut use_node = 0;
        let mut smallest_delay = i64::MAX;
        for (i, node) in enumerate(&*node_vec) {
            let r_node = node.read().await;
            if r_node.is_healthy() {
                return Ok(node.clone());
            }
            let remaining = r_node.remaining_time_for_backoff();
            if remaining < smallest_delay {
                use_node = i;
                smallest_delay = remaining;
            }
        }
        Ok(node_vec[use_node].clone())
    }

    async fn remove_dead_nodes(&self, node_health: &Vec<ManagedNodeHealth>) {
        if self.max_node_attempts > 0 {
            for h in node_health.iter() {
                if h.attempts > self.max_node_attempts {
                    self.remove_node_from_network(&h.key, &h.address).await;
                    let mut nodes = self.nodes.write().await;
                    nodes.remove(h.idx);
                }
            }
        }
    }

    async fn remove_node_from_network(&self, key: &NetworkNodeKey, address: &ManagedNodeAddress) {
        let mut network = self.network.write().await;
        match key {
            NetworkNodeKey::AccountId(_) => {
                let nodes = network.get_mut(key).unwrap();
                if nodes.is_empty() {
                    network.remove(key);
                    return;
                }
                let i = Self::address_is_in_node_list(address, nodes).await;
                if let Some(i) = i {
                    nodes.remove(i);
                    if nodes.is_empty() {
                        network.remove(key);
                    }
                }
            }
            NetworkNodeKey::Address(_) => {
                network.remove(key);
            }
        }
    }
}

// For node ordering
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManagedNodeHealth {
    pub idx: usize,
    pub key: NetworkNodeKey,
    pub address: ManagedNodeAddress,
    pub is_healthy: bool,
    pub last_used: i64,
    pub backoff_until: i64,
    pub use_count: u64,
    pub attempts: u64,
}

impl Ord for ManagedNodeHealth {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_health = self.is_healthy;
        let other_health = other.is_healthy;

        if self_health && !other_health {
            Ordering::Less
        } else if !self_health && other_health {
            Ordering::Greater
        } else {
            if self.use_count < other.use_count {
                Ordering::Less
            } else if self.use_count > other.use_count {
                Ordering::Greater
            } else if self.last_used < other.last_used {
                Ordering::Less
            } else if self.last_used > other.last_used {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        }
    }
}

impl PartialOrd for ManagedNodeHealth {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_node_account_ids_for_execute() {
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

        let max_nodes = nodes.len();
        let mut node_vec = Vec::new();
        for (address, account_id) in nodes.into_iter() {
            node_vec.push(NetworkNode::Node(Node::new(account_id, address, 200)));
        }

        let mut test_network = ManagedNetwork::new_empty();
        test_network.max_nodes_per_transaction = Some(3);
        test_network.set_network_nodes(node_vec).await.unwrap();

        let n_nodes = test_network.number_of_nodes_for_transaction().await;
        assert_eq!(n_nodes, 3);

        let nodes = test_network
            .number_of_most_heathy_nodes(max_nodes + 1)
            .await;
        assert_eq!(nodes.len(), max_nodes);
    }
}
