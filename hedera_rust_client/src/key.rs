use std::convert::TryFrom;

use crate::error::HederaError;
use crate::key_list::KeyList;
use crate::proto::{services, ToProto};
use crate::ContractId;
use crate::PrivateKey;
use crate::PublicKey;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Key {
    Ed25519(PublicKey),
    KeyList(KeyList),
    ThresholdKey(KeyList),
    ContractId(ContractId),
    DelegatableContractId(ContractId),
}

impl ToProto<services::Key> for Key {
    fn to_proto(&self) -> Result<services::Key, HederaError> {
        let key = match &*self {
            Key::Ed25519(key) => key.to_proto()?,
            Key::KeyList(key_list) => key_list.to_proto_key_list_key()?,
            Key::ThresholdKey(key_list) => key_list.to_proto_threshold_key()?,
            Key::ContractId(id) => services::key::Key::ContractId(id.to_proto()?),
            Key::DelegatableContractId(id) => {
                services::key::Key::DelegatableContractId(id.to_proto()?)
            }
        };
        Ok(services::Key { key: Some(key) })
    }
}

impl TryFrom<services::Key> for Key {
    type Error = HederaError;
    fn try_from(services: services::Key) -> Result<Key, Self::Error> {
        match services.key {
            Some(pb_key) => {
                let key = match pb_key {
                    services::key::Key::Ed25519(bytes) => {
                        Key::Ed25519(PublicKey::from_hex_bytes(bytes)?)
                    }
                    services::key::Key::ThresholdKey(key) => {
                        Key::ThresholdKey(KeyList::try_from(key)?)
                    }
                    services::key::Key::KeyList(key) => Key::KeyList(KeyList::try_from(key)?),
                    services::key::Key::ContractId(id) => Key::ContractId(ContractId::from(id)),
                    services::key::Key::DelegatableContractId(id) => {
                        Key::DelegatableContractId(ContractId::from(id))
                    }
                    _ => return Err(HederaError::UnsupportedKeyType),
                };
                Ok(key)
            }
            None => Err(HederaError::NoInnerKey),
        }
    }
}

impl From<PrivateKey> for Key {
    fn from(pk: PrivateKey) -> Key {
        Key::Ed25519(pk.public())
    }
}

impl From<PublicKey> for Key {
    fn from(pk: PublicKey) -> Key {
        Key::Ed25519(pk)
    }
}
