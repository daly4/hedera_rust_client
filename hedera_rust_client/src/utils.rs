use chrono::{DateTime, Duration, Utc};
use lazy_static::lazy_static;
use std::convert::TryFrom;

use crate::key::Key;
use crate::key_list::KeyList;
use crate::proto::services;
use crate::AccountId;
use crate::ContractId;
use crate::FileId;
use crate::ScheduleId;
use crate::TokenId;
use crate::TopicId;
use crate::TransactionId;

lazy_static! {
    pub static ref DEFAULT_DURATION: Duration = Duration::seconds(7890000);
}

optional_try_from!(optional_duration, services::Duration, Duration);
optional_try_from!(optional_timestamp, services::Timestamp, DateTime<Utc>);
optional_try_from!(optional_key, services::Key, Key);
optional_try_from!(optional_key_list, services::KeyList, KeyList);
optional_try_from!(
    optional_transaction_id,
    services::TransactionId,
    TransactionId
);
optional_try_from!(optional_account_id, services::AccountId, AccountId);

non_optional_try_from!(non_optional_timestamp, services::Timestamp, DateTime<Utc>);
non_optional_try_from!(
    non_optional_transaction_id,
    services::TransactionId,
    TransactionId
);
non_optional_try_from!(non_optional_account_id, services::AccountId, AccountId);

non_optional_from!(
    non_optional_contract_id,
    contract_id,
    services::ContractId,
    ContractId
);
non_optional_from!(non_optional_token_id, token_id, services::TokenId, TokenId);
non_optional_from!(non_optional_file_id, file_id, services::FileId, FileId);
non_optional_from!(
    non_optional_schedule_id,
    schedule_id,
    services::ScheduleId,
    ScheduleId
);
non_optional_from!(non_optional_topic_id, topic_id, services::TopicId, TopicId);

#[cfg(test)]
pub mod test_utils {
    use crate::{
        client::{Client, ClientBuilder, Operator},
        mirror_network::MirrorNetwork,
        network::Network,
        AccountId, Hbar, HederaError, NetworkName, PrivateKey, TransactionId, TransferTransaction,
    };
    use chrono::{DateTime, Utc};
    use std::collections::HashMap;

    const MOCK_PRIVATE_KEY: &str = "302e020100300506032b6570042204203b054fade7a2b0869c6bd4a63b7017cbae7855d12acc357bea718e2c3e805962";

    pub fn mock_account_id() -> AccountId {
        AccountId::simple(3)
    }

    pub fn mock_tansaction_valid_start() -> DateTime<Utc> {
        Utc::now()
    }

    pub fn mock_transaction_id() -> TransactionId {
        TransactionId::with_valid_start(mock_account_id(), mock_tansaction_valid_start(), false)
    }

    pub fn mock_private_key() -> PrivateKey {
        MOCK_PRIVATE_KEY.parse().unwrap()
    }

    pub fn mock_client() -> Client {
        let private_key = mock_private_key();

        let mut network = HashMap::new();
        network.insert("nonexistent-testnet".to_string(), AccountId::simple(3));

        let operator = Operator::new(AccountId::simple(2), private_key);

        let network = Network::for_network_name(&NetworkName::TestNet).unwrap();
        let mirror_network = MirrorNetwork::new();

        let client = ClientBuilder::default()
            .operator(operator)
            .network(network)
            .mirror_network(mirror_network)
            .build()
            .expect("failed to build mockclient");

        client
    }

    pub async fn mock_transaction() -> Result<TransferTransaction, HederaError> {
        let test_transaction_id = mock_transaction_id();

        let private_key = mock_private_key();
        let client = mock_client();

        let mut tx = TransferTransaction::new();
        tx.add_hbar_transfer(AccountId::simple(2), Hbar::from_tinybar(-100))?
            .add_hbar_transfer(AccountId::simple(3), Hbar::from_tinybar(100))?
            .set_transaction_id(test_transaction_id.clone())?
            .set_node_account_ids(vec![AccountId::simple(4)])?
            .freeze_with(Some(&client))
            .await?;

        tx.sign(&private_key)?;
        Ok(tx.to_owned())
    }
}
