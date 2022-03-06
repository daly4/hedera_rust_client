mod utils;
use hedera_rust_client::{
    AccountCreateTransaction, AccountDeleteTransaction, Hbar, PrivateKey,
    TransactionId, AccountInfoQuery, AccountId, NetworkName,
};

#[tokio::test]
#[ignore]
async fn test_account_create_delete() {
    let env = utils::IntegrationTestEnv::open().await.unwrap();

    let new_key = PrivateKey::new();
    let initial_balance = Hbar::new(2.0);
    let resp = AccountCreateTransaction::new()
        .set_node_account_ids(env.node_account_ids.clone()).unwrap()
        .set_key(new_key.clone().into()).unwrap()
        .set_initial_balance(initial_balance).unwrap()
        .set_max_automatic_token_associations(100).unwrap()
        .execute(&env.client)
        .await.unwrap();

    let receipt = resp.get_receipt(&env.client).await.unwrap();

    let account_id = receipt.account_id.expect(&format!(
        "no account_id in account create receipt: {:?}",
        receipt
    ));

    let res = AccountDeleteTransaction::new()
        .set_node_account_ids(vec![resp.node_id]).unwrap()
        .set_delete_account_id(account_id).unwrap()
        .set_transfer_account_id(env.client.operator_account_id()).unwrap()
        .set_transaction_id(TransactionId::generate(account_id)).unwrap()
        .freeze_with(Some(&env.client))
        .await.unwrap()
        .sign(&new_key).unwrap()
        .execute(&env.client)
        .await.unwrap();

    let _receipt = res.get_receipt(&env.client).await.unwrap();

    env.close().await.unwrap();
}

#[tokio::test]
async fn test_account_create_no_key() {
    let client = utils::IntegrationTestEnv::testnet_client().unwrap();

    let initial_balance = Hbar::new(2.0);
    let mut resp = AccountCreateTransaction::new();
    let res = resp.set_initial_balance(initial_balance).unwrap()
        .set_max_automatic_token_associations(100).unwrap()
        .execute(&client)
        .await;
    assert!(res.is_err());
}

#[tokio::test]
#[ignore]
async fn test_account_create_set_proxy() {
    let env = utils::IntegrationTestEnv::open().await.unwrap();

    let new_key = PrivateKey::new();
    let initial_balance = Hbar::new(2.0);
    let resp = AccountCreateTransaction::new()
        .set_node_account_ids(env.node_account_ids.clone()).unwrap()
        .set_key(new_key.clone().into()).unwrap()
        .set_initial_balance(initial_balance).unwrap()
        .execute(&env.client)
        .await.unwrap();

    let receipt = resp.get_receipt(&env.client).await.unwrap();

    let account_id = receipt.account_id.expect(&format!(
        "no account_id in account create receipt: {:?}",
        receipt
    ));

    let resp = AccountCreateTransaction::new()
        .set_node_account_ids(env.node_account_ids.clone()).unwrap()
        .set_key(new_key.clone().into()).unwrap()
        .set_initial_balance(initial_balance).unwrap()
        .set_proxy_account_id(account_id).unwrap()
        .execute(&env.client)
        .await.unwrap();

    let receipt = resp.get_receipt(&env.client).await.unwrap();

    let account_id_2 = receipt.account_id.expect(&format!(
        "no account_id in account create receipt: {:?}",
        receipt
    ));

    let info = AccountInfoQuery::new()
        .set_account_id(account_id_2).unwrap()
        .execute(&env.client)
        .await.unwrap();

    let proxy_account_id = info.proxy_account_id.expect(&format!(
        "no proxy_account_id in account query: {:?}",
        info
    ));
    assert_eq!(account_id.to_string(), proxy_account_id.to_string());

    let res = AccountDeleteTransaction::new()
        .set_node_account_ids(env.node_account_ids.clone()).unwrap()
        .set_delete_account_id(account_id).unwrap()
        .set_transfer_account_id(env.client.operator_account_id()).unwrap()
        .set_transaction_id(TransactionId::generate(account_id)).unwrap()
        .freeze_with(Some(&env.client))
        .await.unwrap()
        .sign(&new_key).unwrap()
        .execute(&env.client)
        .await.unwrap();

    let _receipt = res.get_receipt(&env.client).await.unwrap();

    env.close().await.unwrap();
}

#[tokio::test]
#[ignore]
async fn test_account_create_transaction_network() {
    let mut env = utils::IntegrationTestEnv::open().await.unwrap();

    let new_key = PrivateKey::new();
    let initial_balance = Hbar::new(2.0);
    let resp = AccountCreateTransaction::new()
        .set_node_account_ids(env.node_account_ids.clone()).unwrap()
        .set_key(new_key.clone().into()).unwrap()
        .set_initial_balance(initial_balance).unwrap()
        .execute(&env.client)
        .await.unwrap();


    let receipt = resp.get_receipt(&env.client).await.unwrap();

    let account_id = receipt.account_id.expect(&format!(
        "no account_id in account create receipt: {:?}",
        receipt
    ));

    env.client.set_auto_validate_checksums(true);

    let bad_client = utils::IntegrationTestEnv::client(NetworkName::MainNet).unwrap();
    let account_id_string = account_id.to_string_with_checksum(&bad_client).unwrap();
    let account_id: AccountId = account_id_string.parse().unwrap();

    let mut tx = AccountDeleteTransaction::new();
    let res = tx
        .set_node_account_ids(env.node_account_ids.clone()).unwrap()
        .set_delete_account_id(account_id).unwrap()
        .set_transfer_account_id(env.client.operator_account_id()).unwrap()
        .set_transaction_id(TransactionId::generate(account_id)).unwrap()
        .freeze_with(Some(&env.client))
        .await;

    assert!(res.is_err());

    env.close().await.unwrap();
}