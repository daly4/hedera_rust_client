use std::convert::TryFrom;

use crate::error::HederaError;
use crate::key::Key;
use crate::proto::{services, ToProto};
use crate::PublicKey;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyList {
    threshold: Option<u32>,
    keys: Vec<Key>,
}

impl KeyList {
    pub fn new(threshold: Option<u32>) -> KeyList {
        KeyList {
            threshold,
            keys: Vec::new(),
        }
    }

    pub fn of(keys: Vec<Key>) -> KeyList {
        KeyList {
            threshold: None,
            keys,
        }
    }

    pub fn with_threshold(threshold: u32) -> KeyList {
        KeyList::new(Some(threshold))
    }

    pub fn threshold(&self) -> Option<u32> {
        self.threshold
    }

    pub fn set_threshold(&mut self, threshold: u32) {
        self.threshold = Some(threshold);
    }

    pub fn is_empty(&self) -> bool {
        self.keys.is_empty()
    }

    pub fn add(&mut self, key: Key) {
        self.keys.push(key);
    }

    pub fn add_all(&mut self, mut keys: Vec<Key>) {
        self.keys.append(&mut keys);
    }

    pub fn remove(&mut self, key: Key) {
        if let Some(idx) = self.keys.iter().position(move |x| *x == key) {
            self.keys.remove(idx);
        }
    }

    pub fn add_all_public_keys(&mut self, keys: Vec<PublicKey>) {
        for public_key in keys.into_iter() {
            self.add(Key::Ed25519(public_key));
        }
    }

    pub fn to_proto_key_list_key(&self) -> Result<services::key::Key, HederaError> {
        let mut keys = Vec::new();
        for key_val in self.keys.iter() {
            keys.push(key_val.to_proto()?);
        }
        let key_list = services::KeyList { keys };
        Ok(services::key::Key::KeyList(key_list))
    }

    pub fn to_proto_threshold_key(&self) -> Result<services::key::Key, HederaError> {
        match self.threshold {
            Some(t) => {
                let mut keys = Vec::new();
                for key_val in self.keys.iter() {
                    keys.push(key_val.to_proto()?);
                }
                let key_list = services::KeyList { keys };
                Ok(services::key::Key::ThresholdKey(services::ThresholdKey {
                    threshold: t,
                    keys: Some(key_list),
                }))
            }
            None => Err(HederaError::NoThresholdValue),
        }
    }
}

impl ToProto<services::KeyList> for KeyList {
    fn to_proto(&self) -> Result<services::KeyList, HederaError> {
        let mut keys = Vec::new();
        for key_val in self.keys.iter() {
            keys.push(key_val.to_proto()?);
        }
        let key_list = services::KeyList { keys };
        Ok(key_list)
    }
}

impl ToProto<services::Key> for KeyList {
    fn to_proto(&self) -> Result<services::Key, HederaError> {
        let mut keys = Vec::new();
        for key_val in self.keys.iter() {
            keys.push(key_val.to_proto()?);
        }
        let key_list = services::KeyList { keys };
        let key = match self.threshold {
            Some(t) => services::key::Key::ThresholdKey(services::ThresholdKey {
                threshold: t,
                keys: Some(key_list),
            }),
            None => services::key::Key::KeyList(key_list),
        };
        Ok(services::Key { key: Some(key) })
    }
}

impl TryFrom<services::KeyList> for KeyList {
    type Error = HederaError;
    fn try_from(services: services::KeyList) -> Result<KeyList, Self::Error> {
        let mut keys = Vec::new();
        for key_val in services.keys.into_iter() {
            keys.push(Key::try_from(key_val)?);
        }
        Ok(KeyList {
            threshold: None,
            keys,
        })
    }
}

impl TryFrom<services::ThresholdKey> for KeyList {
    type Error = HederaError;
    fn try_from(services: services::ThresholdKey) -> Result<KeyList, Self::Error> {
        Ok(KeyList {
            threshold: Some(services.threshold),
            keys: match services.keys {
                Some(key_list) => {
                    let mut keys = Vec::new();
                    for key_val in key_list.keys.into_iter() {
                        keys.push(Key::try_from(key_val)?);
                    }
                    keys
                }
                None => Vec::new(),
            },
        })
    }
}
