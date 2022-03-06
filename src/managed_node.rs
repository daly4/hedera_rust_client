use chrono::Utc;
use std::cmp::{max, min};
use std::convert::TryInto;
use std::time::Duration;

use crate::managed_node_address::ManagedNodeAddress;

pub fn get_timestamp() -> i64 {
    Utc::now().timestamp()
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManagedNode {
    pub address: ManagedNodeAddress,
    pub current_backoff: i64,
    pub last_used: i64,
    pub backoff_until: i64,
    pub use_count: u64,
    pub min_backoff: u64,
    pub max_backoff: u64,
    pub attempts: u64,
}

impl ManagedNode {
    pub fn new(address: String, min_backoff: u64) -> ManagedNode {
        ManagedNode {
            address: address.parse().unwrap(),
            current_backoff: 250,
            last_used: 0,
            backoff_until: 0,
            use_count: 0,
            min_backoff,
            max_backoff: 8000,
            attempts: 0,
        }
    }

    pub fn in_use(&mut self) {
        self.use_count += 1;
        self.last_used = get_timestamp();
    }

    pub fn is_healthy(&self) -> bool {
        self.backoff_until <= get_timestamp()
    }

    pub fn increase_delay(&mut self) {
        self.attempts = self.attempts + 1;
        self.backoff_until = (self.current_backoff * 100000) + get_timestamp();
        self.current_backoff = min(self.current_backoff * 2, self.max_backoff as i64);
    }

    pub fn decrease_delay(&mut self) {
        self.current_backoff = max(self.current_backoff / 2, self.min_backoff as i64);
    }

    pub fn wait(&self) -> Duration {
        let delay = self.backoff_until - self.last_used;
        Duration::from_nanos(delay.try_into().unwrap())
    }

    pub fn to_secure(&mut self) {
        self.address.to_secure();
    }

    pub fn to_insecure(&mut self) {
        self.address.to_insecure();
    }

    pub fn remaining_backoff(&self) -> i64 {
        self.backoff_until - get_timestamp()
    }
}
