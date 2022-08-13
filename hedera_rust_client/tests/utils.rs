#![allow(dead_code)]

use dotenv::dotenv;
use hedera_rust_client::{
    AccountCreateTransaction, AccountDeleteTransaction, AccountId, Client, ClientBuilder, Hbar,
    HederaError, NetworkName, Operator, PrivateKey, PublicKey, TokenDeleteTransaction, TokenId,
};
use std::collections::HashMap;
use std::env;

#[derive(Debug)]
pub struct IntegrationTestEnv {
    pub client: Client,
    pub operator_key: PrivateKey,
    pub operator_id: AccountId,
    pub origional_operator_key: PublicKey,
    pub origional_operator_id: AccountId,
    pub node_account_ids: Vec<AccountId>,
}

impl IntegrationTestEnv {
    pub fn client(network: NetworkName) -> Result<Client, HederaError> {
        dotenv().ok();
        let private_key: PrivateKey = match env::var("PRIVATE_KEY") {
            Ok(v) => v.parse().unwrap(),
            Err(err) => panic!("env PRIVATE_KEY: {}", err),
        };
        let account_id: AccountId = match env::var("ACCOUNT_ID") {
            Ok(v) => v.parse().unwrap(),
            Err(err) => panic!("env ACCOUNT_ID: {}", err),
        };

        let operator = Operator::new(account_id, private_key);

        let client = ClientBuilder::default()
            .operator(operator)
            .for_network_name(network)?
            .build()
            .unwrap();

        Ok(client)
    }

    pub fn testnet_client() -> Result<Client, HederaError> {
        Self::client(NetworkName::TestNet)
    }

    pub async fn open() -> Result<Self, HederaError> {
        let mut client = Self::testnet_client()?;
        client.set_max_backoff(1000);
        client.set_max_node_attempts(1);

        // prune network
        let mut network = HashMap::new();
        for (address, account_id) in client.network().await.unwrap().into_iter() {
            if (client.ping(account_id).await).is_ok() {
                network.insert(address, account_id);
            }
        }
        if network.is_empty() {
            return Err(HederaError::NoNetworkNodes);
        }
        client.set_network(network).await.unwrap();

        let new_key = PrivateKey::new();
        let initial_balance = Hbar::new(50.0);
        let resp = AccountCreateTransaction::new()
            .set_key(new_key.clone().into())
            .unwrap()
            .set_initial_balance(initial_balance)
            .unwrap()
            .execute(&client)
            .await
            .unwrap();

        let receipt = resp.get_receipt(&client).await.unwrap();
        let account_id = receipt
            .account_id
            .unwrap_or_else(|| panic!("no account_id in account create receipt: {:?}", receipt));

        let origional_operator_id = client.operator_account_id();
        let origional_operator_key = client.operator_public_key();

        let operator = Operator::new(account_id, new_key.clone());
        client.set_operator(operator);

        let node_account_ids = vec![resp.node_id];

        Ok(IntegrationTestEnv {
            client,
            operator_key: new_key,
            operator_id: account_id,
            origional_operator_key,
            origional_operator_id,
            node_account_ids,
        })
    }

    pub async fn close_with_token(self, token_id: TokenId) -> Result<(), HederaError> {
        let _res = TokenDeleteTransaction::new()
            .set_token_id(token_id)?
            .execute(&self.client)
            .await?
            .get_receipt(&self.client)
            .await?;
        self.close().await
    }

    pub async fn close(self) -> Result<(), HederaError> {
        let res = AccountDeleteTransaction::new()
            .set_node_account_ids(self.node_account_ids)?
            .set_delete_account_id(self.operator_id)?
            .set_transfer_account_id(self.origional_operator_id)?
            .execute(&self.client)
            .await?;

        let _ = res.get_receipt(&self.client).await?;
        Ok(())
    }

    pub async fn new_test_account(
        &self,
        initial_balance: Hbar,
    ) -> Result<(AccountId, PrivateKey), HederaError> {
        let new_key = PrivateKey::new();
        let receipt = AccountCreateTransaction::new()
            .set_key(new_key.clone().into())
            .unwrap()
            .set_initial_balance(initial_balance)
            .unwrap()
            .execute(&self.client)
            .await
            .unwrap()
            .get_receipt(&self.client)
            .await
            .unwrap();

        let account_id = receipt
            .account_id
            .unwrap_or_else(|| panic!("no account_id in account create receipt: {:?}", receipt));
        Ok((account_id, new_key))
    }
}
