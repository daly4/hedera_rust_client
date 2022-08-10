use std::time::Duration;
use tonic::transport::Channel as TonicChannel;

use crate::channel::Channel;
use crate::error::HederaError;
use crate::managed_node::ManagedNode;
use crate::proto::mirror::consensus_service_client::ConsensusServiceClient;

pub type ConsensusServiceClientChannel = ConsensusServiceClient<TonicChannel>;

#[derive(Debug, Clone)]
pub struct MirrorNode {
    pub managed_node: ManagedNode,
    pub channel: Option<ConsensusServiceClientChannel>,
}

impl MirrorNode {
    pub fn new(address: String, min_backoff: u64) -> MirrorNode {
        MirrorNode {
            managed_node: ManagedNode::new(address, min_backoff),
            channel: None,
        }
    }

    pub fn channel(&mut self) -> Result<ConsensusServiceClientChannel, HederaError> {
        if let Some(channel) = &self.channel {
            return Ok(channel.clone());
        }
        let tonic_channel = Channel::tonic_channel(&self.managed_node.address.to_string())?;
        let channel = ConsensusServiceClient::new(tonic_channel);
        self.channel = Some(channel.clone());
        Ok(channel)
    }

    pub fn in_use(&mut self) {
        self.managed_node.in_use();
    }

    pub fn is_healthy(&self) -> bool {
        self.managed_node.is_healthy()
    }

    pub fn increase_delay(&mut self) {
        self.managed_node.increase_delay();
    }

    pub fn decrease_delay(&mut self) {
        self.managed_node.decrease_delay();
    }

    pub fn wait(&self) -> Duration {
        self.managed_node.wait()
    }

    pub fn close(&mut self) {
        if self.channel.is_some() {
            self.channel = None;
        }
    }

    pub fn to_secure(&mut self) {
        self.managed_node.to_secure();
    }

    pub fn to_insecure(&mut self) {
        self.managed_node.to_insecure();
    }
}

impl PartialEq for MirrorNode {
    fn eq(&self, other: &Self) -> bool {
        (&self.managed_node, &self.channel.is_some())
            == (&other.managed_node, &other.channel.is_some())
    }
}

impl Eq for MirrorNode {}
