use bytes::Bytes;
use prost::Message;
use std::convert::TryFrom;

use crate::error::HederaError;
use crate::proto::{services, ToProto};

#[derive(Debug, Clone)]
pub struct SignedTransaction {
    pub body_bytes: Vec<u8>,
    pub sig_map: Option<services::SignatureMap>,
}

impl SignedTransaction {
    pub fn with_body_bytes(body_bytes: Vec<u8>) -> SignedTransaction {
        SignedTransaction {
            body_bytes,
            sig_map: None,
        }
    }

    pub fn to_proto_bytes(&self) -> Result<Vec<u8>, HederaError> {
        let body = self.to_proto()?;
        let mut buf = Vec::new();
        buf.reserve(body.encoded_len());
        match body.encode(&mut buf) {
            Ok(_) => Ok(buf),
            Err(e) => Err(HederaError::UnableToSerializeTransaction(e)),
        }
    }

    pub fn try_from_proto_bytes(bytes: Vec<u8>) -> Result<SignedTransaction, HederaError> {
        match services::SignedTransaction::decode(Bytes::from(bytes)) {
            Ok(proto) => Ok(SignedTransaction::try_from(proto)?),
            Err(e) => Err(HederaError::UnableToDeserializeTransaction(e)),
        }
    }

    pub fn add_signature_pair(&mut self, pair: services::SignaturePair) -> Result<(), HederaError> {
        if self.sig_map.is_some() {
            self.sig_map.as_mut().unwrap().sig_pair.push(pair);
            return Ok(());
        }
        let sig_pair = vec![pair];
        self.sig_map = Some(services::SignatureMap { sig_pair });
        Ok(())
    }
}

impl ToProto<services::SignedTransaction> for SignedTransaction {
    fn to_proto(&self) -> Result<services::SignedTransaction, HederaError> {
        Ok(services::SignedTransaction {
            body_bytes: self.body_bytes.clone(),
            sig_map: self.sig_map.clone(),
        })
    }
}

impl TryFrom<services::SignedTransaction> for SignedTransaction {
    type Error = crate::error::HederaError;
    fn try_from(services: services::SignedTransaction) -> Result<SignedTransaction, HederaError> {
        Ok(SignedTransaction {
            body_bytes: services.body_bytes,
            sig_map: services.sig_map,
        })
    }
}
