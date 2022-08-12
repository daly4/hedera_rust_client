use bytes::Bytes;
use chrono::Duration;
use itertools::enumerate;
use prost::Message;
use sha3::{Digest, Sha3_384};
use std::collections::HashMap;
use std::fmt::{self, Debug, Display};

use crate::client::Client;
use crate::error::HederaError;
use crate::executor::{IntermediateResponse, ProtoRequest, Request, Response};
use crate::memo::check_memo_length;
use crate::proto::sdk::TransactionList as ProtoTransactionList;
use crate::proto::services::{
    signature_pair::Signature as ProtoSignature, transaction_body::Data as ProtoData,
    SchedulableTransactionBody as ProtoSchedulableTransactionBody, Transaction as ProtoTransaction,
};
use crate::signed_transaction::SignedTransaction;
use crate::status::Status;
use crate::transaction_body::TransactionBody;
use crate::transaction_id::TransactionId;
use crate::transaction_response::TransactionResponse;
use crate::AccountId;
use crate::Hbar;
use crate::PrivateKey;
use crate::PublicKey;
use crate::ScheduleCreateTransaction;
use crate::Signature;

pub fn transaction_get_node_account_id(request: &Request) -> Result<AccountId, HederaError> {
    let transaction = request.get_transaction()?;
    let account_id: AccountId = transaction.node_account_ids[transaction.next_node_index].clone();
    Ok(account_id)
}

pub fn transaction_make_request(request: &mut Request) -> Result<ProtoRequest, HederaError> {
    let transaction = request.get_transaction_mut()?;
    let index = transaction.node_account_ids.len() * transaction.next_transaction_index
        + transaction.next_node_index;
    transaction.build_transactions(index + 1)?;
    Ok(ProtoRequest::Transaction(
        transaction.transactions[index].clone(),
    ))
}

pub fn transaction_advance_request(request: &mut Request) -> Result<(), HederaError> {
    let transaction = request.get_transaction_mut()?;
    let length = transaction.node_account_ids.len();
    let current_index = transaction.next_node_index;
    transaction.next_node_index = (current_index + 1) % length;
    Ok(())
}

pub fn transaction_map_response_status(
    _request: &Request,
    response: &Response,
) -> Result<Status, HederaError> {
    let transaction = response.get_transaction()?;
    let code = transaction.node_transaction_precheck_code;
    match Status::from_i32(code) {
        Some(v) => Ok(v),
        None => Err(HederaError::UnknownHederaStatusCode(code)),
    }
}

pub fn transaction_should_retry(status: &Status, _response: &Response) -> bool {
    *status == Status::Busy
}

pub fn transaction_map_response(
    mut request: Request,
    _response: Response,
    node_id: AccountId,
    proto_request: ProtoRequest,
) -> Result<IntermediateResponse, HederaError> {
    let mut transaction = request.get_transaction_mut()?;
    let proto_request = proto_request.get_transaction()?;
    let hash = Sha3_384::digest(&proto_request.signed_transaction_bytes);
    let index = transaction.next_transaction_index;
    transaction.next_transaction_index = (index + 1) % transaction.transaction_ids.len();
    return Ok(IntermediateResponse::Transaction(TransactionResponse {
        transaction_id: Some(transaction.transaction_ids[index].clone()),
        scheduled_transaction_id: None,
        node_id,
        hash: hash.to_vec(),
    }));
}

#[derive(Debug, Clone, PartialEq)]
pub struct Transaction {
    transaction_body: TransactionBody,
    next_node_index: usize,
    next_transaction_index: usize,
    pub max_retry: u8,
    pub min_backoff: Option<u64>,
    pub max_backoff: Option<u64>,
    transaction_ids: Vec<TransactionId>,
    transactions: Vec<ProtoTransaction>,
    pub signed_transactions: Vec<SignedTransaction>,
    node_account_ids: Vec<AccountId>,
}

impl Transaction {
    pub fn new() -> Transaction {
        Transaction {
            transaction_body: TransactionBody::new(),
            next_node_index: 0,
            next_transaction_index: 0,
            max_retry: 5,
            min_backoff: None,
            max_backoff: None,
            transaction_ids: Vec::new(),
            transactions: Vec::new(),
            signed_transactions: Vec::new(),
            node_account_ids: Vec::new(),
        }
    }

    pub fn with_max_transaction_fee(fee: Hbar) -> Transaction {
        let mut transaction = Transaction::new();
        transaction
            .set_max_transaction_fee(fee)
            .expect("unable to set max_transaction_fee");
        transaction
    }

    pub fn clear_transactions(&mut self) {
        self.transaction_ids.clear();
        self.transactions.clear();
        self.signed_transactions.clear();
    }

    pub fn to_bytes(&mut self) -> Result<Vec<u8>, HederaError> {
        if !self.is_frozen() {
            return Err(HederaError::TransactionHashMustBeFrozen);
        }
        self.build_transactions(self.signed_transactions.len())?;
        let list = ProtoTransactionList {
            transaction_list: self.transactions.clone(),
        };
        let mut buf = Vec::new();
        buf.reserve(list.encoded_len());
        match list.encode(&mut buf) {
            Ok(_) => Ok(buf),
            Err(e) => return Err(HederaError::TransactionSerializationError(e)),
        }
    }

    pub fn from_bytes(bytes: Vec<u8>) -> Result<Transaction, HederaError> {
        let list = match ProtoTransactionList::decode(Bytes::from(bytes)) {
            Ok(list) => list,
            Err(e) => return Err(HederaError::TransactionDeserializationError(e)),
        };

        let mut tx = Transaction::new();
        tx.transactions = list.transaction_list.clone();

        for transaction in list.transaction_list.iter() {
            let signed_transaction = SignedTransaction::try_from_proto_bytes(
                transaction.signed_transaction_bytes.clone(),
            )?;
            tx.signed_transactions.push(signed_transaction.clone());

            let transaction_body =
                TransactionBody::try_from_proto_bytes(signed_transaction.body_bytes)?;

            if let Some(transaction_id) = transaction_body.transaction_id {
                if !tx.transaction_ids.contains(&transaction_id) {
                    tx.transaction_ids.push(transaction_id.clone());
                }
            }

            if let Some(node_id) = transaction_body.node_account_id {
                if !tx.node_account_ids.contains(&node_id) {
                    tx.node_account_ids.push(node_id.clone());
                }
            }
        }
        Ok(tx)
    }

    pub fn signatures(
        &self,
    ) -> Result<HashMap<AccountId, HashMap<PublicKey, Vec<u8>>>, HederaError> {
        let mut map = HashMap::new();

        if self.signed_transactions.is_empty() {
            return Ok(map);
        }

        for (i, node_account_id) in enumerate(&self.node_account_ids) {
            if let Some(ref sig_ref) = &self.signed_transactions[i].sig_map {
                let mut inner = HashMap::new();
                for sig_pair in &sig_ref.sig_pair {
                    match PublicKey::from_bytes(&sig_pair.pub_key_prefix) {
                        Ok(key) => {
                            let bytes = match &sig_pair.signature {
                                Some(v) => match v {
                                    ProtoSignature::Contract(bytes) => bytes.clone(),
                                    ProtoSignature::Ed25519(bytes) => bytes.clone(),
                                    ProtoSignature::Rsa3072(bytes) => bytes.clone(),
                                    ProtoSignature::Ecdsa384(bytes) => bytes.clone(),
                                    ProtoSignature::EcdsaSecp256k1(bytes) => bytes.clone(),
                                },
                                None => Vec::new(),
                            };
                            inner.insert(key, bytes);
                        }
                        Err(_) => return Err(HederaError::UnableToSerializeKey),
                    }
                }
                map.insert(*node_account_id, inner);
            }
        }
        Ok(map)
    }

    pub fn add_signature(
        &mut self,
        public_key: PublicKey,
        signature: Signature,
    ) -> Result<(), HederaError> {
        self.require_one_node_account_id()?;

        if self.key_already_signed(&public_key) {
            return Ok(());
        }

        if self.signed_transactions.is_empty() {
            return Ok(());
        }

        self.transactions = Vec::new();

        for st in self.signed_transactions.iter_mut() {
            st.add_signature_pair(public_key.to_signature_pair_protobuf(&signature))?;
        }
        Ok(())
    }

    pub fn is_frozen(&self) -> bool {
        !self.signed_transactions.is_empty()
    }

    pub fn require_not_frozen(&self) -> Result<(), HederaError> {
        if self.is_frozen() {
            return Err(HederaError::TransactionImmutable);
        }
        Ok(())
    }

    pub fn require_one_node_account_id(&self) -> Result<(), HederaError> {
        if self.node_account_ids.len() != 1 {
            return Err(HederaError::TransactionNodeAccountIdNotSet);
        }
        Ok(())
    }

    pub fn node_account_ids(&self) -> Vec<AccountId> {
        self.node_account_ids.clone()
    }

    pub fn node_account_ids_len(&self) -> usize {
        self.node_account_ids.len()
    }

    pub fn set_node_account_ids(
        &mut self,
        mut node_account_ids: Vec<AccountId>,
    ) -> Result<(), HederaError> {
        self.require_not_frozen()?;
        self.node_account_ids.append(&mut node_account_ids);
        Ok(())
    }

    pub fn transaction_id(&self) -> Result<TransactionId, HederaError> {
        if !self.transaction_ids.is_empty() || self.is_frozen() {
            return Ok(self.transaction_ids[self.next_transaction_index].clone());
        }
        Err(HederaError::TransactionIdNotFrozen)
    }

    pub fn set_transaction_id(&mut self, transaction_id: TransactionId) -> Result<(), HederaError> {
        self.require_not_frozen()?;
        self.transaction_ids = vec![transaction_id];
        Ok(())
    }

    pub fn add_transaction_id(&mut self, transaction_id: TransactionId) -> Result<(), HederaError> {
        self.require_not_frozen()?;
        self.transaction_ids.push(transaction_id);
        Ok(())
    }

    pub fn set_transaction_body_transaction_id(&mut self, transaction_id: TransactionId) {
        self.transaction_body.transaction_id = Some(transaction_id);
    }

    pub fn init_transaction_id(&mut self, client: Option<&Client>) -> Result<(), HederaError> {
        if self.transaction_ids.is_empty() {
            match client {
                Some(client) => {
                    self.set_transaction_id(TransactionId::generate(client.operator_account_id()))?;
                }
                None => return Err(HederaError::TransactionNodeAccountIdOrClientNotSet),
            }
        }
        let id = self.transaction_id()?;
        self.set_transaction_body_transaction_id(id);
        Ok(())
    }

    pub fn transaction_hash(&mut self) -> Result<Vec<u8>, HederaError> {
        if !self.is_frozen() {
            return Err(HederaError::TransactionHashMustBeFrozen);
        }
        let hashes = self.transaction_hash_per_node()?;

        Ok(hashes.get(&self.node_account_ids[0]).unwrap().clone())
    }

    fn transaction_hash_per_node(&mut self) -> Result<HashMap<AccountId, Vec<u8>>, HederaError> {
        if !self.is_frozen() {
            return Err(HederaError::TransactionHashMustBeFrozen);
        }

        let mut transaction_hash = HashMap::new();
        self.build_transactions(self.signed_transactions.len())?;

        for (i, account_id) in enumerate(&self.node_account_ids) {
            let hash = Sha3_384::digest(&self.transactions[i].signed_transaction_bytes);
            transaction_hash.insert(*account_id, hash.to_vec());
        }
        Ok(transaction_hash)
    }

    fn build_transactions(&mut self, until_index: usize) -> Result<(), HederaError> {
        for i in self.transactions.len()..until_index {
            let transaction = match self.signed_transactions.get(i) {
                Some(v) => {
                    let signed_transaction_bytes = v.to_proto_bytes()?;
                    #[allow(deprecated)]
                    ProtoTransaction {
                        body: None,
                        sigs: None,
                        signed_transaction_bytes,
                        body_bytes: Vec::new(),
                        sig_map: None,
                    }
                }
                None => return Err(HederaError::Unreacahble),
            };
            self.transactions.push(transaction);
        }
        Ok(())
    }

    pub async fn freeze(&mut self) -> Result<(), HederaError> {
        self.freeze_with(None).await
    }

    pub async fn freeze_with(&mut self, client: Option<&Client>) -> Result<(), HederaError> {
        if self.node_account_ids.is_empty() {
            if let Some(client) = client {
                self.node_account_ids = client.node_account_ids_for_execute().await;
                debug_assert!(
                    !self.node_account_ids.is_empty(),
                    "transaction::freeze_with no node_account_ids_for_execute from client"
                );
            } else {
                return Err(HederaError::TransactionNodeAccountIdOrClientNotSet);
            }
        }

        for account_id in self.node_account_ids.iter() {
            let mut body = self.transaction_body.clone();
            body.node_account_id = Some(*account_id);
            let body_bytes = body.to_proto_bytes()?;
            let signed_transaction = SignedTransaction::with_body_bytes(body_bytes);
            self.signed_transactions.push(signed_transaction);
        }
        Ok(())
    }

    // Note - only to be called by chunked tx fn
    pub fn freeze_with_account_id(&mut self, account_id: AccountId) -> Result<(), HederaError> {
        let mut body = self.transaction_body.clone();
        body.node_account_id = Some(account_id);
        let body_bytes = body.to_proto_bytes()?;
        let signed_transaction = SignedTransaction::with_body_bytes(body_bytes);
        self.signed_transactions.push(signed_transaction);
        Ok(())
    }

    pub fn sign(&mut self, private_key: &PrivateKey) -> Result<(), HederaError> {
        self.sign_with(&private_key.public(), |b| private_key.sign(b))?;
        Ok(())
    }

    pub fn sign_with<F: Fn(&Vec<u8>) -> Signature>(
        &mut self,
        public_key: &PublicKey,
        signer: F,
    ) -> Result<(), HederaError> {
        if !self.is_frozen() {
            return Err(HederaError::TransactionSignWithFailed);
        }
        self.transactions.clear();

        if self.key_already_signed(&public_key) {
            return Ok(());
        }

        for signed_transaction in self.signed_transactions.iter_mut() {
            let signature = signer(&signed_transaction.body_bytes);
            signed_transaction
                .add_signature_pair(public_key.to_signature_pair_protobuf(&signature))?;
        }
        Ok(())
    }

    pub async fn sign_with_operator(&mut self, client: &Client) -> Result<(), HederaError> {
        if !self.is_frozen() {
            self.freeze_with(Some(client)).await?;
        }

        let public_key = client.operator_public_key_ref();
        if self.key_already_signed(public_key) {
            return Ok(());
        }

        let tx_signer = client.operator_transaction_signer();
        self.sign_with(public_key, |b| tx_signer(b))
    }

    fn key_already_signed(&self, pk: &PublicKey) -> bool {
        if !self.signed_transactions.is_empty() && self.signed_transactions[0].sig_map.is_some() {
            let pk_bytes = pk.as_bytes_vec();
            for pair in &self.signed_transactions[0]
                .sig_map
                .as_ref()
                .unwrap()
                .sig_pair
            {
                if pk_bytes == pair.pub_key_prefix {
                    return true;
                }
            }
        }
        false
    }

    pub fn max_transaction_fee(&self) -> Hbar {
        self.transaction_body.transaction_fee
    }

    // sets the max transaction fee for this Transaction.
    pub fn set_max_transaction_fee(&mut self, fee: Hbar) -> Result<(), HederaError> {
        self.require_not_frozen()?;
        self.transaction_body.transaction_fee = fee;
        Ok(())
    }

    pub fn init_fee(&mut self, client: Option<&Client>) -> Result<(), HederaError> {
        if let Some(client) = client {
            if self.max_transaction_fee().is_zero() {
                self.set_max_transaction_fee(client.max_transaction_fee())?;
            }
        }
        Ok(())
    }

    pub fn transaction_memo(&self) -> String {
        self.transaction_body.memo.clone()
    }

    // sets the memo for this Transaction.
    pub fn set_transaction_memo(&mut self, memo: String) -> Result<(), HederaError> {
        self.require_not_frozen()?;
        check_memo_length(&memo)?;
        self.transaction_body.memo = memo;
        Ok(())
    }

    pub fn transaction_valid_duration(&self) -> Option<Duration> {
        self.transaction_body.transaction_valid_duration
    }

    pub fn set_transaction_valid_duration(
        &mut self,
        duration: Option<Duration>,
    ) -> Result<(), HederaError> {
        self.require_not_frozen()?;
        self.transaction_body.transaction_valid_duration = duration;
        Ok(())
    }

    pub fn set_transaction_body_data(&mut self, data: Option<ProtoData>) {
        self.transaction_body.data = data;
    }

    // must call on_freeze before calling
    pub fn scheduled(&mut self) -> Result<ScheduleCreateTransaction, HederaError> {
        if self.transaction_body.data.is_none() {
            return Err(HederaError::TransactionBodyNotSet);
        }

        if !self.node_account_ids.is_empty() {
            return Err(HederaError::ScheduledTransactionNodeAccountIdsSet);
        }

        if !self.transaction_ids.is_empty() {
            self.transaction_ids[0].scheduled = true;
            self.set_transaction_body_transaction_id(self.transaction_ids[0].clone())
        }

        let schedulable = ProtoSchedulableTransactionBody::try_from(self.transaction_body.clone())?;
        let mut scheduled = ScheduleCreateTransaction::new();
        scheduled.set_schedulable_transaction_body(schedulable)?;
        if !self.transaction_ids.is_empty() {
            scheduled.set_transaction_id(self.transaction_ids[0].clone())?;
        }
        Ok(scheduled)
    }

    pub fn body(&self) -> &TransactionBody {
        &self.transaction_body
    }
}

impl Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:x?}",
            self.signed_transactions[0]
                .to_proto_bytes()
                .unwrap_or(Vec::new())
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::test_utils::*;

    #[test]
    fn test_transaction_id() {
        let test_tx_id = mock_transaction_id();
        let mut tx = Transaction::new();
        tx.set_transaction_id(test_tx_id.clone()).unwrap();

        let tx_id = tx.transaction_id().unwrap();
        assert_eq!(tx_id, test_tx_id);
    }

    #[test]
    fn test_transaction_node_account_id() {
        let test_node_ids = vec![mock_account_id()];
        let mut tx = Transaction::new();
        tx.set_node_account_ids(test_node_ids.clone()).unwrap();
        let node_account_ids = tx.node_account_ids();
        assert_eq!(node_account_ids, test_node_ids);
    }

    #[tokio::test]
    async fn test_tansaction_serialization_deserialization() {
        let mut transaction = mock_transaction().await.unwrap();
        transaction.freeze().await.unwrap();
        transaction.signatures().unwrap();
        transaction.transaction_hash().unwrap();
        let bytes = transaction.to_bytes().unwrap();
        let deserialized_tx = Transaction::from_bytes(bytes).unwrap();
        assert_eq!(transaction.to_string(), deserialized_tx.to_string());
    }
}
