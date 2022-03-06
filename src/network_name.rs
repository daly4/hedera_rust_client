use serde::Deserialize;
use std::str::FromStr;

use crate::error::HederaError;

#[derive(Debug, Clone, Deserialize)]
pub enum NetworkName {
    MainNet,
    TestNet,
    PreviewNet,
    Other,
}

impl FromStr for NetworkName {
    type Err = HederaError;
    fn from_str(input: &str) -> Result<NetworkName, Self::Err> {
        match input {
            "mainnet" => Ok(NetworkName::MainNet),
            "testnet" => Ok(NetworkName::TestNet),
            "previewnet" => Ok(NetworkName::PreviewNet),
            "other" => Ok(NetworkName::Other),
            _ => Err(HederaError::UnknownNetworkType(input.to_string())),
        }
    }
}
