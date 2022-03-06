use serde::Deserialize;
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;
use std::sync::Arc;

use crate::crypto::{PrivateKey, PublicKey, Signature};
use crate::error::HederaError;
use crate::hbar::Hbar;
use crate::ledger_id::LedgerId;
use crate::managed_network::ArcNetworkNode;
use crate::mirror_network::MirrorNetwork;
use crate::network::Network;
use crate::network_name::NetworkName;
use crate::proto::services;
use crate::AccountBalance;
use crate::AccountBalanceQuery;
use crate::AccountId;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OperatorConfig {
    account_id: String,
    private_key: String,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum NetworkConfig {
    Str(String),
    Map(HashMap<String, String>),
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum MirrorNetworkConfig {
    Str(String),
    StrVec(Vec<String>),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientConfig {
    network: NetworkConfig,
    #[serde(default)]
    operator: Option<OperatorConfig>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    mirror_network: Option<MirrorNetworkConfig>,
}

impl ClientConfig {
    pub fn new(path: &str) -> Result<ClientConfig, HederaError> {
        let mut config = config::Config::default();
        config.merge(config::File::with_name(path))?;
        Ok(config.try_into()?)
    }
}

#[derive(Clone)]
pub struct Operator {
    pub account_id: AccountId,
    pub public_key: PublicKey,
    pub transaction_signer: Arc<dyn Fn(&Vec<u8>) -> Signature + Send + Sync + 'static>,
}

impl Operator {
    pub fn new(account_id: AccountId, private_key: PrivateKey) -> Operator {
        Operator {
            account_id,
            public_key: private_key.public(),
            transaction_signer: Arc::new(move |b| private_key.sign(b)),
        }
    }
}

impl std::fmt::Debug for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Operator")
            .field("account_id", &self.account_id)
            .field("public_key", &self.public_key)
            .finish()
    }
}

impl std::fmt::Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.account_id, self.public_key)
    }
}

#[derive(Builder, Debug, Clone)]
pub struct Client {
    operator: Operator,
    #[builder(default = "Hbar::new(1.0)")]
    max_transaction_fee: Hbar,
    #[builder(default = "Hbar::new(1.0)")]
    max_query_payment: Hbar,
    network: Network,
    mirror_network: MirrorNetwork,
    #[builder(default = "false")]
    auto_validate_checksums: bool,
    #[builder(default = "None")]
    max_attempts: Option<u8>,
    #[builder(default = "250")]
    min_backoff: u64,
    #[builder(default = "8000")]
    max_backoff: u64,
}

impl Client {
    pub fn new(operator: Operator, max_transaction_fee: Hbar, max_query_payment: Hbar) -> Client {
        Client {
            operator,
            max_transaction_fee,
            max_query_payment,
            network: Network::new(),
            mirror_network: MirrorNetwork::new(),
            auto_validate_checksums: false,
            max_attempts: None,
            min_backoff: 250,
            max_backoff: 8000,
        }
    }

    pub fn set_operator(&mut self, operator: Operator) {
        self.operator = operator;
    }

    pub fn set_max_backoff(&mut self, max: u64) {
        if max > self.min_backoff {
            self.max_backoff = max;
        }
    }

    pub fn max_backoff(&self) -> u64 {
        self.max_backoff
    }

    pub fn set_min_backoff(&mut self, min: u64) {
        if min < self.max_backoff {
            self.min_backoff = min;
        }
    }
    
    pub fn min_backoff(&self) -> u64 {
        self.min_backoff
    }
    
    pub fn set_max_attempts(&mut self, max_attempts: u8) {
        self.max_attempts = Some(max_attempts);
    }

    pub fn max_attempts(&self) -> Option<u8> {
        self.max_attempts
    }

    pub fn set_max_node_attempts(&mut self, max_node_attempts: u64) {
        self.network.set_max_node_attempts(max_node_attempts);
    }

    pub fn max_node_attempts(&self) -> u64 {
        self.network.max_node_attempts()
    }

    pub fn set_node_min_backoff(&mut self, node_min_backoff: u64) {
        self.network.set_node_min_backoff(node_min_backoff);
    }

    pub fn node_min_backoff(&self) -> u64 {
        self.network.node_min_backoff()
    }

    pub fn set_node_max_backoff(&mut self, node_max_backoff: u64) {
        self.network.set_node_max_backoff(node_max_backoff);
    }

    pub fn node_max_backoff(&self) -> u64 {
        self.network.node_max_backoff()
    }

    pub fn set_max_nodes_per_transaction(&mut self, max_nodes_per_transaction: usize) {
        self.network.set_max_nodes_per_transaction(max_nodes_per_transaction);
    }

    pub fn max_nodes_per_transaction(&self) -> Option<usize> {
        self.network.max_nodes_per_transaction()
    }

    pub fn set_mirror_network(&mut self, mirror_network: MirrorNetwork) {
        self.mirror_network = mirror_network;
    }

    pub async fn set_transport_security(&mut self, tls: bool) {
        self.network.set_transport_security(tls).await;
        self.mirror_network.set_transport_security(tls).await;
    }

    pub fn set_certificate_verification(&mut self, verify: bool) {
        self.network.set_verify_certificate(verify);
    }

    pub fn certificate_verification(&self) -> bool {
        self.network.verify_certificate()
    }

    pub async fn set_network_name(&mut self, net: NetworkName) -> Result<(), HederaError> {
        self.network.set_network_name(net).await
    }

    pub fn network_name(&self) -> NetworkName {
        self.network.ledger_id().as_network_name()
    }

    pub async fn set_ledger_id(&mut self, id: LedgerId) -> Result<(), HederaError> {
        self.network.set_ledger_id(id).await
    }

    pub fn ledger_id(&self) -> LedgerId {
        self.network.ledger_id()
    }

    pub fn set_auto_validate_checksums(&mut self, validate: bool) {
        self.auto_validate_checksums = validate;
    }
    
    pub fn auto_validate_checksums(&self) -> bool {
        self.auto_validate_checksums
    }

    pub fn operator_account_id(&self) -> AccountId {
        self.operator.account_id
    }

    pub fn operator_public_key(&self) -> PublicKey {
        self.operator.public_key.clone()
    }

    pub fn max_transaction_fee(&self) -> Hbar {
        self.max_transaction_fee
    }

    pub fn max_query_payment(&self) -> Hbar {
        self.max_query_payment
    }

    pub async fn node_account_ids_for_execute(&self) -> Vec<AccountId> {
        self.network.node_account_ids_for_execute().await
    }

    pub async fn network(&self) -> Result<HashMap<String, AccountId>, HederaError> {
        self.network.network().await
    }

    pub async fn set_network(&mut self, network: HashMap<String, AccountId>) -> Result<(), HederaError> {
        self.network.set_network(network).await
    }

    pub async fn node_for_account_id(
        &self,
        node_account_id: &AccountId,
    ) -> Result<ArcNetworkNode, HederaError> {
        self.network.node_for_account_id(node_account_id).await
    }

    pub async fn next_mirror_node(&self) -> Result<ArcNetworkNode, HederaError> {
        self.mirror_network.get_next_mirror_node().await
    }

    pub fn sign_with_operator(&self, bytes: &Vec<u8>) -> Signature {
        (self.operator.transaction_signer)(bytes)
    }

    pub fn to_signature_pair_protobuf(&self, signature: &Signature) -> services::SignaturePair {
        self.operator
            .public_key
            .to_signature_pair_protobuf(signature)
    }

    pub fn operator_public_key_ref(&self) -> &PublicKey {
        &self.operator.public_key
    }

    pub fn operator_transaction_signer(
        &self,
    ) -> Arc<dyn Fn(&Vec<u8>) -> Signature + Send + Sync + 'static> {
        self.operator.transaction_signer.clone()
    }

    pub async fn ping(
        &self,
        node_account_id: AccountId,
    ) -> Result<AccountBalance, HederaError> {
        let balance = AccountBalanceQuery::new()
            .set_account_id(node_account_id)?
            .set_node_account_ids(vec![node_account_id])?
            .execute(self)
            .await?;
        Ok(balance)
    }

    pub async fn ping_all(&self) -> Result<(), HederaError> {
        let nodes = self.network().await?;
        for (_, node) in nodes.into_iter() {
            self.ping(node).await?;
        }
        Ok(())
    }
}

impl ClientBuilder {
    pub fn from_config_file(path: Option<&str>) -> Result<ClientBuilder, HederaError> {
        match path {
            Some(path) => {
                let cfg = ClientConfig::new(path)?;
                let builder = Self::from_config(cfg)?;
                Ok(builder)
            }
            None => Err(HederaError::NoFileProvided),
        }
    }

    pub fn from_config(cfg: ClientConfig) -> Result<ClientBuilder, HederaError> {
        let mut builder = ClientBuilder::default();
        let (network, mirror_network) = match cfg.network {
            NetworkConfig::Str(network_name) => {
                let network_name = NetworkName::from_str(&network_name)?;
                (
                    Network::for_network_name(&network_name)?,
                    MirrorNetwork::for_network_name(&network_name)?,
                )
            }
            NetworkConfig::Map(nodes) => {
                let mut network = HashMap::new();
                for (key, val) in nodes.iter() {
                    network.insert(val.clone(), AccountId::from_str(key)?);
                }
                (
                    Network::from_network(network)?,
                    MirrorNetwork::from_network(Vec::new())?,
                )
            }
        };
        builder.network = Some(network);
        builder.mirror_network = Some(mirror_network);

        // if mirror network defined then override
        if let Some(mirror_network_cfg) = cfg.mirror_network {
            let mirror_network = match mirror_network_cfg {
                MirrorNetworkConfig::Str(network_name) => {
                    let network_name = NetworkName::from_str(&network_name)?;
                    MirrorNetwork::for_network_name(&network_name)?
                }
                MirrorNetworkConfig::StrVec(nodes) => MirrorNetwork::from_network(nodes)?,
            };
            builder.mirror_network = Some(mirror_network);
        }

        if let Some(operator_cfg) = cfg.operator {
            let operator = Operator::new(
                AccountId::from_str(&operator_cfg.account_id)?,
                PrivateKey::from_str(&operator_cfg.private_key)?,
            );
            builder.operator = Some(operator);
        }

        Ok(builder)
    }

    pub fn for_network_name(
        &mut self,
        network_name: NetworkName,
    ) -> Result<&mut Self, HederaError> {
        self.network = Some(Network::for_network_name(&network_name)?);
        self.mirror_network = Some(MirrorNetwork::for_network_name(&network_name)?);
        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    const CONFIG_WITH_OPERATOR: &str = r#"
    {
        "network": {
            "0.0.21": "0.testnet.hedera.com:50211"
        },
        "operator": {
            "accountId": "0.0.21",
            "privateKey": "302e020100300506032b657004220420db484b828e64b2d8f12ce3c0a0e93a0b8cce7af1bb8f39c97732394482538e10"
        }
    }"#;

    const CONFIG_NO_OPERATOR: &str = r#"
    {
        "network": "mainnet"
    }"#;

    const CONFIG_WITH_MIRROR_NETWORK: &str = r#"
    {
        "network": "mainnet",
        "operator": {
            "accountId": "0.0.36",
            "privateKey": "302e020100300506032b657004220420db484b828e64b2d8f12ce3c0a0e93a0b8cce7af1bb8f39c97732394482538e10"
        },
        "mirrorNetwork": "mainnet"
    }"#;

    #[test]
    fn deserialize_client_config() {
        let _cfg: ClientConfig = serde_json::from_str(CONFIG_WITH_OPERATOR).unwrap();
        let _cfg: ClientConfig = serde_json::from_str(CONFIG_NO_OPERATOR).unwrap();
        let _cfg: ClientConfig = serde_json::from_str(CONFIG_WITH_MIRROR_NETWORK).unwrap();
    }

    #[test]
    fn from_config_with_operator() {
        let cfg: ClientConfig = serde_json::from_str(CONFIG_WITH_OPERATOR).unwrap();
        let builder = ClientBuilder::from_config(cfg).unwrap();
        assert!(builder.network.is_some());
        assert!(builder.mirror_network.is_some());
        assert!(builder.operator.is_some());
        builder.build().unwrap();
    }

    #[test]
    fn from_config_no_operator() {
        let cfg: ClientConfig = serde_json::from_str(CONFIG_NO_OPERATOR).unwrap();
        let builder = ClientBuilder::from_config(cfg).unwrap();
        assert!(builder.network.is_some());
        assert!(builder.mirror_network.is_some());
        assert!(builder.operator.is_none());

        let client = builder.build();
        assert!(client.is_err(), "should fail with no operator");
    }
}
