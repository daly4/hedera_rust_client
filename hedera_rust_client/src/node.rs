use std::time::Duration;

use crate::channel::Channel;
use crate::error::HederaError;
use crate::managed_node::ManagedNode;
use crate::node_address::NodeAddress;
use crate::AccountId;

#[derive(Debug, Clone)]
pub struct Node {
    pub managed_node: ManagedNode,
    pub account_id: AccountId,
    pub channel: Option<Channel>,
    pub address_book: Option<NodeAddress>,
    pub verify_certificate: bool,
}

impl Node {
    pub fn new(account_id: AccountId, address: String, min_backoff: u64) -> Node {
        let managed_node = ManagedNode::new(address, min_backoff);
        Node {
            managed_node,
            account_id,
            channel: None,
            address_book: None,
            verify_certificate: true,
        }
    }

    pub fn channel(&mut self) -> Result<Channel, HederaError> {
        if let Some(c) = &self.channel {
            return Ok(c.clone());
        }

        let channel = if self.verify_certificate && self.address_book.is_some() {
            Channel::from_authority_tls(
                &self.managed_node.address.to_string(),
                &self
                    .address_book
                    .as_ref()
                    .ok_or(HederaError::MissingNodeCertHash)?
                    .cert_hash,
            )?
        } else {
            Channel::from_authority(&self.managed_node.address.to_string())?
        };

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

    pub fn remaining_backoff(&self) -> i64 {
        self.managed_node.remaining_backoff()
    }
}
